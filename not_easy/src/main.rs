fn main() {
    let vtuber_data = "\
    vtuber name,age (year)
    Kizuna AI, 7
    C酱です,10
    You,8
    Invalid,data
    ";
 
    let records = vtuber_data.lines();
 
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',')
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug info: {:?} -> {:?}", record, fields)
        }

        let name = fields[0];

        if let Ok(age) = fields[1].parse::<f32>() {
            println!("{}, {}years", name, age)
        }
    }
  }

// debug info: "    Kizuna AI, 7" -> ["Kizuna AI", "7"]
// Kizuna AI, 7years
// debug info: "    C酱です,10" -> ["C酱です", "10"]
// C酱です, 10years
// debug info: "    You,8" -> ["You", "8"]
// You, 8years
// debug info: "    Invalid,data" -> ["Invalid", "data"]
 