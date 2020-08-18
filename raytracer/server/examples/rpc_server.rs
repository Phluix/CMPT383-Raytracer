// Port of https://www.rabbitmq.com/tutorials/tutorial-six-python.html. Start this
// example in one shell, then the rpc_client example in another.
use amiquip::{
    AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish,
    QueueDeclareOptions, Result,
};

use serde_json::Value;

fn fib(input_string: String) -> String {
    // Convert input string into JSON 
    let input_JSON: Value = serde_json::from_str(&input_string).unwrap();

    // Get arrays of each field as json Value object
    let ground_JSON = input_JSON["ground"].as_array().unwrap();
    let skycolor_JSON = input_JSON["sky_color"].as_array().unwrap();
    let lookfrom_JSON = input_JSON["lookfrom"].as_array().unwrap();
    let lookat_JSON = input_JSON["lookat"].as_array().unwrap();
    let vup_JSON = input_JSON["vup"].as_array().unwrap();


    let sphere1_center_JSON = input_JSON["sphere1_center"].as_array().unwrap();
    let sphere2_center_JSON = input_JSON["sphere2_center"].as_array().unwrap();
    let sphere3_center_JSON = input_JSON["sphere3_center"].as_array().unwrap();

    let sphere2_color_JSON = input_JSON["sphere2_color"].as_array().unwrap();
    let sphere3_color_JSON = input_JSON["sphere3_color"].as_array().unwrap();
    

    // Create mutable vectors
    let mut ground: Vec<f32> = Vec::new();
    let mut skycolor: Vec<f32> = Vec::new();

    let mut lookfrom: Vec<f32> = Vec::new();
    let mut lookat: Vec<f32> = Vec::new();
    let mut vup: Vec<f32> = Vec::new();

    let mut sphere1_center: Vec<f32> = Vec::new();
    let mut sphere2_center: Vec<f32> = Vec::new();
    let mut sphere3_center: Vec<f32> = Vec::new();

    let mut sphere2_color: Vec<f32> = Vec::new();
    let mut sphere3_color: Vec<f32> = Vec::new();

    let sphere1_radius: f32 = input_JSON["sphere1_radius"].as_f64().unwrap() as f32;
    let sphere2_radius: f32 = input_JSON["sphere2_radius"].as_f64().unwrap() as f32;
    let sphere3_radius: f32 = input_JSON["sphere3_radius"].as_f64().unwrap() as f32;

    // Fill mutable vectors with parsed values
    for g in ground_JSON {
        ground.push(g.as_f64().unwrap() as f32);
    } 

    for s in skycolor_JSON {
        skycolor.push(s.as_f64().unwrap() as f32);
    } 

    for l in lookfrom_JSON {
        lookfrom.push(l.as_f64().unwrap() as f32);
    } 

    for l in lookat_JSON {
        lookat.push(l.as_f64().unwrap() as f32);
    } 

    for v in vup_JSON {
        vup.push(v.as_f64().unwrap() as f32);
    } 

    for s in sphere1_center_JSON {
        sphere1_center.push(s.as_f64().unwrap() as f32);
    } 

    for s in sphere2_center_JSON {
        sphere2_center.push(s.as_f64().unwrap() as f32);
    } 

    for s in sphere3_center_JSON {
        sphere3_center.push(s.as_f64().unwrap() as f32);
    } 

    for s in sphere2_color_JSON {
        sphere2_color.push(s.as_f64().unwrap() as f32);
    }

    for s in sphere3_color_JSON {
        sphere3_color.push(s.as_f64().unwrap() as f32);
    }

    let scene = raytracer::Scene {
        ground,
        skycolor,
        sphere1_center,
        sphere1_radius,
        sphere2_center,
        sphere2_radius,
        sphere2_color,
        sphere3_center,
        sphere3_radius,
        sphere3_color,
        lookfrom,
        lookat,
        vup
    };

    let output = raytracer::render(scene);

    return output;
}

fn main() -> Result<()> {
    //env_logger::init();

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the default direct exchange.
    let exchange = Exchange::direct(&channel);

    // Declare the queue that will receive RPC requests.
    let queue = channel.queue_declare("rpc_queue", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Awaiting RPC requests");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) fib({})", i, body);

                let (reply_to, corr_id) = match (
                    delivery.properties.reply_to(),
                    delivery.properties.correlation_id(),
                ) {
                    (Some(r), Some(c)) => (r.clone(), c.clone()),
                    _ => {
                        println!("received delivery without reply_to or correlation_id");
                        consumer.ack(delivery)?;
                        continue;
                    }
                };

                let response = match body.parse() {
                    Ok(n) => format!("{}", fib(n)),
                    Err(_) => "invalid input".to_string(),
                };

                exchange.publish(Publish::with_properties(
                    response.as_bytes(),
                    reply_to,
                    AmqpProperties::default().with_correlation_id(corr_id),
                ))?;
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}