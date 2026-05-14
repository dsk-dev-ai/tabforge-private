#[derive(Debug)]

pub struct Worker {

    pub id:u32,

    pub active:bool
}

pub fn initialize_workers() {

    let workers = vec![

        Worker{
            id:1,
            active:true
        },

        Worker{
            id:2,
            active:true
        }

    ];

    println!(
        "[WORKERS] Pool initialized"
    );

    println!(
        "[WORKERS] {} workers online",
        workers.len()
    );
}