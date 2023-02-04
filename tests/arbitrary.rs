#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use rand::{
        distributions::{Alphanumeric, DistString},
        rngs::StdRng,
        Rng, SeedableRng,
    };
    use tabled::{
        style::{HorizontalLine, Line, VerticalLine},
        Style, Table, Tabled,
    };

    use super::*;

    #[derive(Tabled, Clone, Debug)]
    struct DataFixture {
        one: String,
        two: i8,
        three: u8,
        four: u16,
        five: u32,
        six: u64,
        seven: u128,
        eight: i8,
        nine: i16,
        ten: i32,
        eleven: i64,
        twelve: i128,
        thirteen: bool,
    }

    impl Arbitrary for DataFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            DataFixture {
                one: Alphanumeric
                    .sample_string(&mut rng, u8::arbitrary(g) as usize)
                    .to_string(), //String::arbitrary(g), //Alphanumeric.sample_string(&mut rng, 100), //Arbitrary::arbitrary(g),
                //one: Arbitrary::arbitrary(g),
                two: Arbitrary::arbitrary(g),
                three: Arbitrary::arbitrary(g),
                four: Arbitrary::arbitrary(g),
                five: Arbitrary::arbitrary(g),
                six: Arbitrary::arbitrary(g),
                seven: Arbitrary::arbitrary(g),
                eight: Arbitrary::arbitrary(g),
                nine: Arbitrary::arbitrary(g),
                ten: Arbitrary::arbitrary(g),
                eleven: Arbitrary::arbitrary(g),
                twelve: Arbitrary::arbitrary(g),
                thirteen: Arbitrary::arbitrary(g),
            }
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_data(data: Vec<DataFixture>) {
        //TODO - randomize style too
        let data_len = data.len();
        /*         let styles = [
            Style::empty(),
            Style::blank(),
            Style::ascii(),
            Style::psql(),
            Style::markdown(),
            Style::modern(),
            Style::sharp(),
            Style::rounded(),
            Style::extended(),
            Style::dots(),
            Style::re_structured_text(),
            Style::ascii_rounded(),
        ];
        let mut rng = rand::thread_rng();
        let raw_style = styles[(rng.gen() as usize % styles.len()) - 1]; */

        let horizontals: Vec<HorizontalLine> = (1..=data_len)
            .map(|i| HorizontalLine::new(i, Line::full('*', 'i', 's', 'e')))
            .collect();
        let verticals: Vec<VerticalLine> = (0..=13)
            .map(|i| VerticalLine::new(i, Line::filled('c')))
            .collect();
        let style = Style::modern()
            .horizontals(horizontals)
            .verticals(verticals)
            .bottom('b')
            .top('t');

        let table = Table::new(data).with(style).to_string();
        let lines: Vec<String> = table.lines().map(|l| l.into()).collect();

        let border_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('s'))
            .map(String::to_string)
            .collect();

        let data_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('c'))
            .map(String::to_string)
            .collect();

        if data_len > 0 {
            println!("{}", table);
            println!("Data len: {}", data_len);
            assert_eq!(data_len, border_lines.len());
            assert_eq!(data_len + 1, data_lines.len());

            assert!(border_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 's' && last == 'e'
            }));

            assert!(data_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 'c' && last == 'c'
            }));
        }
    }

    #[test]
    fn test_data_example() {
        /*  let mut data = vec![
            DataFixture {
                one: "abc".into(),
                two: 123,
                three: 345,
            },
            DataFixture {
                one: "abcd".into(),
                two: 1234,
                three: 3456,
            },
            DataFixture {
                one: "abcde".into(),
                two: 12345,
                three: 34567,
            },
        ]; */

        let mut data = vec![
            DataFixture { 
                one: "R3Egx6XH9BhNzLoapymq9ik3q2A9Swn1VoMjGb96Zgt4arFUUrM3NbeU82hdkJ5AVqiYItKzcYiGC8sC1Gsfww4HTeVNq6LULWi3zmgNA1hNh24PdX5glo5eCp66PK276YcoIN3jMXvaAW0tHUt7dJOJIUKfkkVxEn0MzA".into(), 
                two: -101, three: 101, four: 4470, five: 400723926, six: 1918391416998183760, seven: 140054211636082994519104882202515779614,
                 eight: -120, nine: -135, ten: -1602386255, eleven: 1137669194203252582, twelve: -153266831407786241301746557558731341075, thirteen: false 
                },
        ];
        //let data: Vec<DataFixture> = vec![];

        //TODO - randomize style too
        let data_len = data.len();
        let horizontals: Vec<HorizontalLine> = (1..=data_len)
            .map(|i| HorizontalLine::new(i, Line::full('*', 'i', 's', 'e')))
            .collect();
        let verticals: Vec<VerticalLine> = (0..=13)
            .map(|i| VerticalLine::new(i, Line::filled('c')))
            .collect();
        let style = Style::modern()
            .horizontals(horizontals)
            .verticals(verticals)
            .bottom('b')
            .top('t');

        let table = Table::new(data).with(style).to_string();
        let lines: Vec<String> = table.lines().map(|l| l.into()).collect();
        let border_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('s'))
            .map(String::to_string)
            .collect();

        let data_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('c'))
            .map(String::to_string)
            .collect();

        println!("{}", data_len);
        //println!("{:?}", border_lines);
        //println!("{:?}", data_lines);

        println!("{}", table);

        if data_len > 0 {
            assert_eq!(data_len, border_lines.len());
            assert_eq!(data_len + 1, data_lines.len());

            /* assert!(border_lines.iter_mut().all(|l| {
                let last = l.pop().unwrap();
                let first = l.remove(0);
                first == 's' && last == 'e'
            }));

            assert!(data_lines.iter_mut().all(|l| {
                let last = l.pop().unwrap();
                let first = l.remove(0);
                first == 'c' && last == 'c'
            })); */

            border_lines.iter().for_each(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                println!("First: {}, Last: {}", first, last);
                //first == 's' && last == 'e'
            });

            data_lines.iter().for_each(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                println!("Data First: {}, Data Last: {}", first, last);
                //first == 's' && last == 'e'
            });

            assert!(border_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 's' && last == 'e'
            }));

            assert!(data_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 'c' && last == 'c'
            }));
        }

        //println!("{:?}", data_lines);

        /* let result = &lines.filter(|l| l.starts_with("s")).all(|l| {
            println!("String contains s or e: {}", l.contains("s|e"));
            l.contains("s") && l.contains("e")
        });
        println!("Result {}", result); */
    }
    /*
    #[test]
    fn test_data_example_failing() {
        let mut data = vec![DataFixture {
            one: "hGWFp3r".into(),
            two: 7220890409807117749,
            three: 3523361437,
        }];

        /*
        tttttttttttttttttttttttttttttttttttttttttttttt
        c one     c two                 │ three      │
        s*********i*********************i************e
        c hGWFp3r c 7220890409807117749 │ 3523361437 │
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb

        */

        //data = vec![];

        //TODO - randomize style too
        let data_len = data.len();
        let horizontals: Vec<HorizontalLine> = (1..=data_len)
            .map(|i| HorizontalLine::new(i, Line::full('*', 'i', 's', 'e')))
            .collect();
        let verticals: Vec<VerticalLine> = (0..=data_len)
            //.map(|i| VerticalLine::new(i, Line::filled('c')))
            .map(|i| VerticalLine::new(i, Line::full('*', 'i', 's', 'e')))
            .collect();
        let style = Style::modern()
            .horizontals(horizontals)
            .verticals(verticals)
            .bottom('b')
            .top('t');

        let table = Table::new(data).with(style).to_string();
        let lines: Vec<String> = table.lines().map(|l| l.into()).collect();
        let border_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('s'))
            .map(String::to_string)
            .collect();

        let data_lines: Vec<String> = lines
            .iter()
            .filter(|&l| l.starts_with('c'))
            .map(String::to_string)
            .collect();

        println!("{}", data_len);
        //println!("{:?}", border_lines);
        //println!("{:?}", data_lines);

        println!("{}", table);

        if data_len > 0 {
            assert_eq!(data_len, border_lines.len());
            assert_eq!(data_len + 1, data_lines.len());

            /* assert!(border_lines.iter_mut().all(|l| {
                let last = l.pop().unwrap();
                let first = l.remove(0);
                first == 's' && last == 'e'
            }));

            assert!(data_lines.iter_mut().all(|l| {
                let last = l.pop().unwrap();
                let first = l.remove(0);
                first == 'c' && last == 'c'
            })); */

            border_lines.iter().for_each(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                println!("First: {}, Last: {}", first, last);
                //first == 's' && last == 'e'
            });

            data_lines.iter().for_each(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                println!("Data First: {}, Data Last: {}", first, last);
                //first == 's' && last == 'e'
            });

            assert!(border_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 's' && last == 'e'
            }));

            assert!(data_lines.iter().all(|l| {
                let mut chars = l.chars();
                let first = chars.nth(0).unwrap();
                let last = chars.rev().nth(0).unwrap();
                first == 'c' && last == 'c'
            }));
        }

        //println!("{:?}", data_lines);

        /* let result = &lines.filter(|l| l.starts_with("s")).all(|l| {
            println!("String contains s or e: {}", l.contains("s|e"));
            l.contains("s") && l.contains("e")
        });
        println!("Result {}", result); */
    } */


    
}
