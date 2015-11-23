
pub struct City {
	id:i16,
	pid:i16,
	lvl:i8,
	names:Vec<String>
}

pub fn all_citys() -> Vec<City> {
	let mut citys:Vec<City> = vec![];
	citys.push(City{id:1, pid:0, lvl:1, names:vec!["甘肃".to_string(), "甘肃省".to_string()]});
	citys.push(City{id:2, pid:1, lvl:2, names:vec!["甘南藏族".to_string(), "甘南藏族自治州".to_string(), "甘南".to_string(), "甘南自治州".to_string()]});
	citys.push(City{id:3, pid:2, lvl:3, names:vec!["碌曲".to_string(), "碌曲县".to_string()]});
	citys.push(City{id:4, pid:3, lvl:4, names:vec!["玛艾".to_string(), "玛艾镇".to_string()]});
	citys.push(City{id:5, pid:3, lvl:4, names:vec!["阿拉".to_string(), "阿拉乡".to_string()]});
	citys
}