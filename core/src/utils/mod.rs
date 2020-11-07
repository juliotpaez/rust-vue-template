use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

pub async fn using_lock<'a, L, F: FnOnce(&mut MutexGuard<L>) -> R, R>(lock: &Mutex<L>, f: F) -> R {
    let mut lock_guard: MutexGuard<L> = lock.lock().await;
    f(&mut lock_guard)
}