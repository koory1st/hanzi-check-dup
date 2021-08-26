const SKIP_STR: &str = "，。；";

fn main() {
    let learned = "云对雨，雪对风，晚照对晴空。来鸿对去燕，宿鸟对鸣虫。三尺剑，六钧弓，岭北对江东。人间清暑殿，天上广寒宫。两岸晓烟杨柳绿，一园春雨杏花红。两鬓风霜，途次早行之客；一蓑烟雨，溪边晚钓之翁。".to_string();
    let unlearned = "沿对革，异对同，白叟对黄童。江风对海雾，牧子对渔翁。颜巷陋，阮途穷，冀北对辽东。池中濯足水，门外打头风。梁帝讲经同泰寺，汉皇置酒未央宫。尘虑萦心，懒抚七弦绿绮；霜华满鬓，羞看百炼青铜。".to_string();

    let unique_learned = get_unique(&learned);

    let unique_unlearned_origin = get_unique(&unlearned);

    let rt = get_diff(unique_unlearned_origin, unique_learned);
    
    println!("{:?}", rt);
}

fn get_diff(p0: Vec<char>, p1: Vec<char>) -> Vec<char> {
    let mut rt = Vec::new();

    for to_add in p0 {
        if !p1.contains(&to_add) {
            rt.push(to_add);
        }
    }

    rt
}

fn get_unique(input: &str) -> Vec<char> {
    let mut rt = Vec::new();
    for char in input.chars() {
        if SKIP_STR.contains(char) {
            continue;
        }
        if rt.contains(&char) {
            continue;
        }
        rt.push(char);
    }
    rt
}
