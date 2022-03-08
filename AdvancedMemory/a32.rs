const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

fn main() {
    let data: Vec<&str> = MOCK_DATA.split('\n').skip(1).collect();
    let names = Names {
        inner: data
            .iter()
            .filter_map(|line| line.split(',').nth(1))
            .collect(),
    };

    let titles = Titles {
        inner: data
            .iter()
            .filter_map(|line| line.split(',').nth(4))
            .collect(),
    };

    let data = names.inner.iter().zip(titles.inner.iter());
    for (name, title) in data {
        println!("Name: '{}'; Title: '{}'", name, title);
    }
}
