use leptos::{component, create_signal, view, IntoView, SignalSetter, SignalWith};

pub struct Todo {
    title: String,
    completed: bool,
}

pub struct Todos(Vec<Todo>);

impl Todos {
    pub fn num_remaining(&self) -> usize {
        self.0.iter().filter(|todo| !todo.completed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaining() {
        let todos = Todos(vec![
            Todo {
                title: "Task 1".to_string(),
                completed: false,
            },
            Todo {
                title: "Task 2".to_string(),
                completed: true,
            },
            Todo {
                title: "Task 3".to_string(),
                completed: false,
            },
        ]);
        assert_eq!(todos.num_remaining(), 2);
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(Todos(vec![
        Todo {
            title: "Learn Leptos".to_string(),
            completed: false,
        },
        Todo {
            title: "Build an app".to_string(),
            completed: false,
        },
    ]));

    let num_remaining = move || todos.with(Todos::num_remaining);

    view! {
        <div>
            <h1>"Todo App"</h1>
            <p>"Remaining tasks: " {num_remaining}</p>
            // <ButtonA set_todos={set_todos.into()} />
        </div>
    }
}

// #[component]
// fn ButtonA(set_todos: SignalSetter<Todos>) -> impl IntoView {
//     let on_click = move |_| {
//         set_todos.set(|todos| {
//             let mut new_todos = Todos(todos.0.clone());
//             new_todos.0.push(Todo {
//                 title: "New Task".to_string(),
//                 completed: false,
//             });
//             new_todos
//         });
//     };

//     view! {
//         <button on:click=on_click>"Add Task"</button>
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TodoApp />
    }
}

fn main() {
    leptos::mount_to_body(App)
}
