// crates/rydit-rs/src/modules/quest.rs
// Quest System - Sistema de Misiones para RyDit
// v0.13.1 - Objetivos, progreso, recompensas

use blast_core::{Executor, Valor};
use ry_parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::eval::evaluar_expr;

// ============================================================================
// QUEST STRUCT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuestState {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct QuestObjective {
    pub description: String,
    pub target: i32,
    pub current: i32,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub state: QuestState,
    pub objectives: Vec<QuestObjective>,
    pub rewards: HashMap<String, f32>,
    pub giver_entity: Option<String>,
    pub turn_in_entity: Option<String>,
}

impl Quest {
    pub fn new(id: &str, title: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            state: QuestState::NotStarted,
            objectives: Vec::new(),
            rewards: HashMap::new(),
            giver_entity: None,
            turn_in_entity: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.objectives.iter().all(|o| o.completed) && self.state == QuestState::InProgress
    }

    pub fn progress_percent(&self) -> f64 {
        if self.objectives.is_empty() {
            return 0.0;
        }
        let completed = self.objectives.iter().filter(|o| o.completed).count() as f64;
        completed / self.objectives.len() as f64 * 100.0
    }
}

// ============================================================================
// QUEST MANAGER
// ============================================================================

thread_local! {
    static QUEST_MANAGER: RefCell<QuestManager> = RefCell::new(QuestManager::new());
}

pub struct QuestManager {
    pub quests: HashMap<String, Quest>,
    pub active_quests: Vec<String>,
    pub completed_quests: Vec<String>,
    pub failed_quests: Vec<String>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: HashMap::new(),
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            failed_quests: Vec::new(),
        }
    }

    pub fn create_quest(&mut self, id: &str, title: &str, description: &str) -> String {
        let quest = Quest::new(id, title, description);
        self.quests.insert(id.to_string(), quest);
        format!("Quest '{}' creada", id)
    }

    pub fn add_objective(&mut self, quest_id: &str, description: &str, target: i32) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            q.objectives.push(QuestObjective {
                description: description.to_string(),
                target,
                current: 0,
                completed: false,
            });
            Ok(format!("Objetivo añadido a quest '{}'", quest_id))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn set_reward(&mut self, quest_id: &str, reward_type: &str, amount: f32) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            q.rewards.insert(reward_type.to_string(), amount);
            Ok(format!("Recompensa {}={} para quest '{}'", reward_type, amount, quest_id))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn set_giver(&mut self, quest_id: &str, entity_id: &str) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            q.giver_entity = Some(entity_id.to_string());
            Ok(format!("NPC '{}' es dador de quest '{}'", entity_id, quest_id))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn start_quest(&mut self, quest_id: &str) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            if q.state != QuestState::NotStarted {
                return Err(format!("Quest '{}' ya fue iniciada", quest_id));
            }
            q.state = QuestState::InProgress;
            self.active_quests.push(quest_id.to_string());
            Ok(format!("Quest '{}' iniciada", quest_id))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn complete_quest(&mut self, quest_id: &str) -> Result<Valor, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            if q.state != QuestState::InProgress {
                return Err(format!("Quest '{}' no está en progreso", quest_id));
            }
            if !q.is_complete() {
                return Err(format!("Quest '{}' no tiene objetivos completados", quest_id));
            }
            q.state = QuestState::Completed;
            self.active_quests.retain(|id| id != quest_id);
            self.completed_quests.push(quest_id.to_string());

            let rewards_text: Vec<String> = q.rewards.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
            Ok(Valor::Texto(format!(
                "Quest '{}' completada! Recompensas: {}",
                quest_id,
                rewards_text.join(", ")
            )))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn fail_quest(&mut self, quest_id: &str) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            q.state = QuestState::Failed;
            self.active_quests.retain(|id| id != quest_id);
            self.failed_quests.push(quest_id.to_string());
            Ok(format!("Quest '{}' fallida", quest_id))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn update_progress(&mut self, quest_id: &str, objective_index: usize, amount: i32) -> Result<String, String> {
        if let Some(q) = self.quests.get_mut(quest_id) {
            if q.state != QuestState::InProgress {
                return Err(format!("Quest '{}' no está en progreso", quest_id));
            }
            if objective_index >= q.objectives.len() {
                return Err(format!("Objetivo {} no existe en quest '{}'", objective_index, quest_id));
            }
            let obj = &mut q.objectives[objective_index];
            obj.current += amount;
            if obj.current >= obj.target {
                obj.current = obj.target;
                obj.completed = true;
            }
            Ok(format!("Progreso quest '{}': objetivo {} = {}/{}", quest_id, objective_index, obj.current, obj.target))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn get_quest_state(&self, quest_id: &str) -> Result<Valor, String> {
        if let Some(q) = self.quests.get(quest_id) {
            let s = match q.state {
                QuestState::NotStarted => "not_started",
                QuestState::InProgress => "in_progress",
                QuestState::Completed => "completed",
                QuestState::Failed => "failed",
            };
            Ok(Valor::Texto(s.to_string()))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn get_progress(&self, quest_id: &str) -> Result<Valor, String> {
        if let Some(q) = self.quests.get(quest_id) {
            let state_str = match q.state {
                QuestState::NotStarted => "no iniciada",
                QuestState::InProgress => "en progreso",
                QuestState::Completed => "completada",
                QuestState::Failed => "fallida",
            };
            let objs: String = q.objectives.iter().enumerate().map(|(i, o)| {
                format!("  {}: {}/{} {}", i, o.current, o.target, if o.completed { "✓" } else { "..." })
            }).collect::<Vec<_>>().join("\n");
            Ok(Valor::Texto(format!(
                "Quest: {}\nEstado: {}\nProgreso: {:.0}%\nObjetivos:\n{}",
                q.title, state_str, q.progress_percent(), objs
            )))
        } else {
            Err(format!("Quest '{}' no encontrada", quest_id))
        }
    }

    pub fn list_active_quests(&self) -> Valor {
        let quests: Vec<Valor> = self.active_quests.iter().filter_map(|id| {
            self.quests.get(id).map(|q| Valor::Texto(format!("{}: {:.0}%", q.title, q.progress_percent())))
        }).collect();
        Valor::Array(quests)
    }

    pub fn list_completed_quests(&self) -> Valor {
        Valor::Array(self.completed_quests.iter().map(|id| Valor::Texto(id.clone())).collect())
    }

    pub fn quest_count(&self, state_filter: &str) -> Valor {
        let count = match state_filter {
            "active" => self.active_quests.len(),
            "completed" => self.completed_quests.len(),
            "failed" => self.failed_quests.len(),
            _ => self.quests.len(),
        };
        Valor::Num(count as f64)
    }
}

fn with_manager<F, R>(f: F) -> R where F: FnOnce(&mut QuestManager) -> R {
    QUEST_MANAGER.with(|qm| f(&mut qm.borrow_mut()))
}

// ============================================================================
// FUNCIONES EXPUESTAS A .RYDIT
// ============================================================================

pub fn quest_create<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() < 3 { return Valor::Error("quest::create(id, titulo, descripcion) requiere 3 argumentos".to_string()); }
    let id = evaluar_expr(&args[0], executor, funcs);
    let title = evaluar_expr(&args[1], executor, funcs);
    let desc = evaluar_expr(&args[2], executor, funcs);
    if let (Valor::Texto(id), Valor::Texto(title), Valor::Texto(desc)) = (id, title, desc) {
        with_manager(|qm| Valor::Texto(qm.create_quest(&id, &title, &desc)))
    } else { Valor::Error("quest::create requiere 3 textos".to_string()) }
}

pub fn quest_add_objective<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() < 3 { return Valor::Error("quest::add_objective(quest_id, descripcion, target) requiere 3 argumentos".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    let desc = evaluar_expr(&args[1], executor, funcs);
    let target = evaluar_expr(&args[2], executor, funcs);
    if let (Valor::Texto(qid), Valor::Texto(desc), Valor::Num(target)) = (qid, desc, target) {
        with_manager(|qm| Valor::Texto(qm.add_objective(&qid, &desc, target as i32).unwrap_or_else(|e| e)))
    } else { Valor::Error("quest::add_objective requiere (texto, texto, numero)".to_string()) }
}

pub fn quest_set_reward<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() < 3 { return Valor::Error("quest::set_reward(quest_id, tipo, cantidad) requiere 3 argumentos".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    let rtype = evaluar_expr(&args[1], executor, funcs);
    let amount = evaluar_expr(&args[2], executor, funcs);
    if let (Valor::Texto(qid), Valor::Texto(rtype), Valor::Num(amount)) = (qid, rtype, amount) {
        with_manager(|qm| Valor::Texto(qm.set_reward(&qid, &rtype, amount as f32).unwrap_or_else(|e| e)))
    } else { Valor::Error("quest::set_reward requiere (texto, texto, numero)".to_string()) }
}

pub fn quest_start<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() != 1 { return Valor::Error("quest::start(quest_id) requiere 1 argumento".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(qid) = qid {
        with_manager(|qm| Valor::Texto(qm.start_quest(&qid).unwrap_or_else(|e| e)))
    } else { Valor::Error("quest::start requiere texto".to_string()) }
}

pub fn quest_complete<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() != 1 { return Valor::Error("quest::complete(quest_id) requiere 1 argumento".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(qid) = qid {
        with_manager(|qm| qm.complete_quest(&qid).unwrap_or_else(|e| Valor::Error(e)))
    } else { Valor::Error("quest::complete requiere texto".to_string()) }
}

pub fn quest_update_progress<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() < 3 { return Valor::Error("quest::update_progress(quest_id, objective, amount) requiere 3 argumentos".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    let idx = evaluar_expr(&args[1], executor, funcs);
    let amt = evaluar_expr(&args[2], executor, funcs);
    if let (Valor::Texto(qid), Valor::Num(idx), Valor::Num(amt)) = (qid, idx, amt) {
        with_manager(|qm| Valor::Texto(qm.update_progress(&qid, idx as usize, amt as i32).unwrap_or_else(|e| e)))
    } else { Valor::Error("quest::update_progress requiere (texto, numero, numero)".to_string()) }
}

pub fn quest_get_progress<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    if args.len() != 1 { return Valor::Error("quest::get_progress(quest_id) requiere 1 argumento".to_string()); }
    let qid = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(qid) = qid {
        with_manager(|qm| qm.get_progress(&qid).unwrap_or_else(|e| Valor::Error(e)))
    } else { Valor::Error("quest::get_progress requiere texto".to_string()) }
}

pub fn quest_list_active<'a>(_args: &[Expr<'a>], _executor: &mut Executor, _funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    with_manager(|qm| qm.list_active_quests())
}

pub fn quest_list_completed<'a>(_args: &[Expr<'a>], _executor: &mut Executor, _funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    with_manager(|qm| qm.list_completed_quests())
}

pub fn quest_count<'a>(args: &[Expr<'a>], executor: &mut Executor, funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>) -> Valor {
    let state = if args.is_empty() {
        "all".to_string()
    } else {
        match evaluar_expr(&args[0], executor, funcs) {
            Valor::Texto(s) => s,
            _ => "all".to_string(),
        }
    };
    with_manager(|qm| qm.quest_count(&state))
}
