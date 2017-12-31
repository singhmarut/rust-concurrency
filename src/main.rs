    use std::thread;
    use std::sync::mpsc;
    use std::time::{Duration,Instant};
    use std::collections::HashMap;


    extern crate futures;
    extern crate tokio_core;
    extern crate tokio_io;

    use futures::{Future, Stream};
    use tokio_io::{io, AsyncRead};
    use tokio_core::net::TcpListener;
    use tokio_core::reactor::Core;

    mod network;


    #[derive(Clone)]
    struct Item {
        created_at: Instant,
        id:i64,
        pub description: String
    }

    impl Item {

        pub fn new(id: i64,description: String) -> Item {
            Item {
                created_at: Instant::now(),
                id: id,
                description: description
            }
        }

        fn created_at(&self) -> Instant {
            self.created_at
        }

        fn id(&self) -> i64 {
            self.id
        }
    }


    fn main() {
        let (sender, receiver) = mpsc::channel(); //Creat  multiple publisher single receiver channel
        let sender_pop = sender.clone(); //clone sender so that

        //Create a thread that sends pop every 2 seconds
        thread::spawn(move || {
            //Create infinite loop
            loop {
                thread::sleep(Duration::from_millis(100));
                sender_pop.send(Item::new(-1,String::from("Pop"))).unwrap();
            }
        });

        //Create a thread that keeps sending data every second t
        thread::spawn(move || {
            let mut val = 1;
            //Create infinite loop
            loop {
                val = val + 1;
                sender.send(Item::new(val,String::from("New"))).unwrap();
                thread::sleep(Duration::from_millis(100));
                //Break out of loop if you want to
                if val == 5 {
                    println!("OK, that's enough");
                    // Exit this loop
                    break;
                }
            }
        });

        let mut vals: HashMap<i64,Item> = HashMap::new(); //Create a mutable vector
        let ttl = 5; //TTL in seconds
        //Receive items in non blocking fashion
//        for received in receiver {
//            //let item = &received;
//            let mut item = &received;
//            let newItem: Item  = item.clone();//Item::new(item.id,item.description);
//            match item.description.as_ref(){
//                "Pop" => {
//                    println!("Pop");
//                    let vals_copy = vals.clone();
//                   // vals.retain(|ref id, x| Instant::now().duration_since(x.created_at).as_secs() < ttl);
//
//                    for (id, item) in &vals_copy {
//                        if Instant::now().duration_since(item.created_at).as_secs() < ttl {
//                            vals.remove(id);
//                        }
//                        println!("{}:", id);
//                    }
//
//                   // vals.iter().for_each(|id, x| println!("{:?}", x.id) )
//
//
////                    let newVals: Vec<&Item> = vals.iter().filter(|x| Instant::now().duration_since(x.created_at).as_secs() < 5)
////                        .inspect(|x| println!("made it through filter: {:?}", x.id))
//                },
//                _ => {
//                    vals.insert(newItem.id,newItem);
//                }
//            }
//        }

        let mut core = Core::new().unwrap();
        network::network::launch_tcp_server();
    }
