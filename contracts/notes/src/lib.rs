
#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

const TASKS: Symbol = symbol_short!("TASKS");
const COUNTER: Symbol = symbol_short!("COUNT");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // Ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah task baru
    pub fn learn(env: Env, title: String) -> String {

        // Ambil data tasks lama
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        // Ambil counter ID
        let mut count: u64 = env.storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        // Increment ID
        count += 1;

        // Buat task baru
        let task = Task {
            id: count,
            title,
            completed: false,
        };

        // Simpan task
        tasks.push_back(task);

        // Update storage
        env.storage().instance().set(&TASKS, &tasks);
        env.storage().instance().set(&COUNTER, &count);

        String::from_str(&env, "Task berhasil ditambahkan")
    }

    // Update status completed
    pub fn toggle_task(env: Env, id: u64) -> String {

        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {

            let mut task = tasks.get(i).unwrap();

            if task.id == id {

                task.completed = !task.completed;

                tasks.set(i, task);

                env.storage().instance().set(&TASKS, &tasks);

                return String::from_str(
                    &env,
                    "Status task berhasil diupdate"
                );
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    // Hapus task
    pub fn delete_task(env: Env, id: u64) -> String {

        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {

            let task = tasks.get(i).unwrap();

            if task.id == id {

                tasks.remove(i);

                env.storage().instance().set(&TASKS, &tasks);

                return String::from_str(
                    &env,
                    "Task berhasil dihapus"
                );
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}

mod test;