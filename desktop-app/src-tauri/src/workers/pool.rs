#[derive(
    Debug,
    Clone
)]

pub struct Worker{

    pub id:u32,

    pub active:bool,

    pub assigned_tab:
        Option<String>,

    pub cpu_usage:f32,

    pub stream_count:u32
}



pub fn initialize_workers(){

    let workers=vec![

        Worker{

            id:1,

            active:true,

            assigned_tab:
                None,

            cpu_usage:
                2.3,

            stream_count:
                0
        },

        Worker{

            id:2,

            active:true,

            assigned_tab:
                None,

            cpu_usage:
                1.8,

            stream_count:
                0
        }

    ];


    println!(
        "[WORKERS] Pool initialized"
    );

    println!(
        "[WORKERS] {} workers online",
        workers.len()
    );


    for worker in workers{

        println!(
            "[WORKER {}]",
            worker.id
        );

        println!(
            "Status: {}",

            if worker.active{
                "ACTIVE"
            }else{
                "OFFLINE"
            }

        );

        println!(
            "Assigned Tab: {:?}",
            worker.assigned_tab
        );

        println!(
            "CPU Usage: {}%",
            worker.cpu_usage
        );

        println!(
            "Streams: {}",
            worker.stream_count
        );

        println!(
            "----------------"
        );

    }

}