#[cfg(test)]
mod tests {

    use std::{
        fs::{File, OpenOptions},
        io::Write,
    };

    use quickcheck::Arbitrary;
    use sqlx::testing::TestFixture;
    use tabled::{builder::Builder, object::Cell, style::VerticalLine, Modify, Span, Style};

    use super::*;

    #[derive(Clone, Debug)]
    struct TableStructure {
        pub rows: Vec<Line>,
        pub theme: ThemeFixture,
        pub row_span: Vec<u8>,
        pub col_span: Vec<u8>,
    }

    //FIXME u8
    type Line = Vec<u32>;

    #[derive(Clone, Debug)]
    pub enum ThemeFixture {
        Empty,
        Blank,
        Ascii,
        Psql,
        Markdown,
        Modern,
        Sharp,
        Rounded,
        Extended,
        Dots,
        RestructuredText,
        AsciiRounded,
    }
    impl Arbitrary for TableStructure {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                rows: Arbitrary::arbitrary(g),
                theme: ThemeFixture::arbitrary(g),
                row_span: Arbitrary::arbitrary(g),
                col_span: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for ThemeFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            use ThemeFixture::*;
            g.choose(&[
                Empty,
                Blank,
                Ascii,
                Psql,
                Markdown,
                Modern,
                Sharp,
                Rounded,
                Extended,
                Dots,
                RestructuredText,
                AsciiRounded,
            ])
            .unwrap()
            .to_owned()
        }
    }

    /*   WRONG
    #[quickcheck_macros::quickcheck]
      fn test_data_string(table_structure: TableStructure) {
          let mut file = OpenOptions::new()
              .create(true)
              .append(true)
              .write(true)
              .read(true)
              .open("table.txt")
              .unwrap();

          //FIXME remove clone
          let rows = table_structure.clone().rows;
          let theme = table_structure.clone().theme;
          let test_count = rows.len();
          let row_count = rows.first().unwrap_or(&vec![]).len();
          let col_count = rows.first().unwrap_or(&vec![]).len();

          println!("Test Count      : {}", test_count);
          println!("Rows Count      : {}", row_count);
          println!("Column Count    : {}", col_count);

          let mut builder = Builder::default();
          for row in rows.iter() {
              for col in row.iter() {
                  let col: Vec<String> = col.iter().map(|e| e.to_string()).collect(); //TODO Change
                  builder.add_record(col); //TODO Change
              }
          }

          builder.set_columns((1..rows.len() + 1).map(|head| head.to_string()));
          let mut table = builder.build();

          use ThemeFixture::*;
          match theme {
              Empty => {
                  table.with(Style::empty());
              }
              Blank => {
                  table.with(Style::blank());
              }
              Ascii => {
                  table.with(Style::ascii());
              }
              Psql => {
                  table.with(Style::psql());
              }
              Markdown => {
                  table.with(Style::markdown());
              }
              Modern => {
                  table.with(Style::modern());
              }
              Sharp => {
                  table.with(Style::sharp());
              }
              Rounded => {
                  table.with(Style::rounded());
              }
              Extended => {
                  table.with(Style::extended());
              }
              Dots => {
                  table.with(Style::dots());
              }
              RestructuredText => {
                  table.with(Style::re_structured_text());
              }
              AsciiRounded => {
                  table.with(Style::ascii_rounded());
              }
          }

          let output = table.to_string();
          //println!("{:?}", table_structure);

          writeln!(file, "                                                                                                                             ");
          writeln!(file, "                                                                                                                             ");
          file.write_all(format!("Test Count : {}", test_count).as_bytes())
              .unwrap();
          file.write_all(format!("Row Count    : {}", row_count).as_bytes())
              .unwrap();
          file.write_all(format!("Column Count : {}", col_count).as_bytes())
              .unwrap();
          writeln!(file, "                                                                                                                             ");
          file.write_all(format!("{:?}", table_structure).as_bytes())
              .unwrap();
          //file.write_all(output.as_bytes()).unwrap();

          //println!("{}", output);
          let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
          let line_width =
              tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
          println!("Line width: {}", line_width);
          assert!(table_lines
              .iter()
              .all(|l| tabled::papergrid::util::string_width(l) == line_width));
          println!("Assert passed");
          println!("{}", "*".repeat(100))
      } */

    #[test]
    fn test_data_example() {
        //Test Count : 25Row Count    : 60Column Count : 7
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .read(true)
            .open("table.txt")
            .unwrap();

        let data1: Vec<Vec<u32>> = vec![
            vec![01, 02, 03, 04, 05],
            vec![11, 12, 13, 14, 15],
            vec![21, 22, 23, 24, 25],
            vec![31, 32, 33, 34, 35],
        ];

        let data2: Vec<Vec<u32>> = vec![
            vec![081, 082, 083],
            vec![811, 812, 813],
            vec![821, 822, 823],
            vec![831, 832, 833],
        ];

        /*    let data1: Vec<Vec<String>> = vec![
            vec!["01", "02", "03", "04", "05"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["11", "12", "13", "14", "15"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["21", "22", "23", "24", "25"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["31", "32", "33", "34", "35"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
        ];

        let data2: Vec<Vec<String>> = vec![
            vec!["081", "082", "083"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["811", "812", "813"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["821", "822", "823"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["831", "832", "833"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
        ]; */
        let table_structure = TableStructure {
            rows: data2,
            theme: ThemeFixture::Modern,
            row_span: vec![],
            col_span: vec![],
        };

        let rows = table_structure.clone().rows;
        let theme = table_structure.clone().theme;
        let row_count = rows.len();
        let col_count = rows.first().unwrap_or(&vec![]).len();

        println!("Rows Count      : {}", row_count);
        println!("Column Count    : {}", col_count);

        let mut builder = Builder::default();
        for r in rows.iter() {
            let col_count = r.len();
            builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
        }

        builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
        let mut table = builder.build();

        use ThemeFixture::*;
        match theme {
            Empty => {
                table.with(Style::empty());
            }
            Blank => {
                table.with(Style::blank());
            }
            Ascii => {
                table.with(Style::ascii());
            }
            Psql => {
                table.with(Style::psql());
            }
            Markdown => {
                table.with(Style::markdown());
            }
            Modern => {
                table.with(Style::modern());
            }
            Sharp => {
                table.with(Style::sharp());
            }
            Rounded => {
                table.with(Style::rounded());
            }
            Extended => {
                table.with(Style::extended());
            }
            Dots => {
                table.with(Style::dots());
            }
            RestructuredText => {
                table.with(Style::re_structured_text());
            }
            AsciiRounded => {
                table.with(Style::ascii_rounded());
            }
        }

        let output = table.to_string();
        //println!("{:?}", table_structure);

        /* writeln!(file, "                                                                                                                             ");
               writeln!(file, "                                                                                                                             ");
               file.write_all(format!("Test Count : {}", test_count).as_bytes())
                   .unwrap();
               file.write_all(format!("Row Count    : {}", row_count).as_bytes())
                   .unwrap();
               file.write_all(format!("Column Count : {}", col_count).as_bytes())
                   .unwrap();
               writeln!(file, "                                                                                                                             ");
               file.write_all(format!("{:?}", table_structure).as_bytes())
                   .unwrap();
               //file.write_all(output.as_bytes()).unwrap();
        */
        println!("{}", output);
        let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
        let line_width =
            tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
        println!("Line width: {}", line_width);
        assert!(table_lines
            .iter()
            .all(|l| tabled::papergrid::util::string_width(l) == line_width));
        println!("Assert passed");
        println!("{}", "*".repeat(100))
    }

    #[quickcheck_macros::quickcheck]
    fn qc_test_data_string(table_structure: TableStructure) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .read(true)
            .open("table.txt")
            .unwrap();

        let rows = table_structure.clone().rows;
        let theme = table_structure.clone().theme;
        let row_count = rows.len();
        let col_count = rows
            .iter()
            .map(|r| r.len())
            .reduce(usize::max)
            .unwrap_or_default();

        println!("Rows Count      : {}", row_count);
        println!("Column Count    : {}", col_count);

        let mut builder = Builder::default();
        for r in rows.iter() {
            builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
        }

        builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
        let mut table = builder.build();

        use ThemeFixture::*;
        match theme {
            Empty => {
                table.with(Style::empty());
            }
            Blank => {
                table.with(Style::blank());
            }
            Ascii => {
                table.with(Style::ascii());
            }
            Psql => {
                table.with(Style::psql());
            }
            Markdown => {
                table.with(Style::markdown());
            }
            Modern => {
                table.with(Style::modern());
            }
            Sharp => {
                table.with(Style::sharp());
            }
            Rounded => {
                table.with(Style::rounded());
            }
            Extended => {
                table.with(Style::extended());
            }
            Dots => {
                table.with(Style::dots());
            }
            RestructuredText => {
                table.with(Style::re_structured_text());
            }
            AsciiRounded => {
                table.with(Style::ascii_rounded());
            }
        }

        let output = table.to_string();
        //println!("{:?}", table_structure);

        writeln!(file, "                                                                                                                             ");
        writeln!(file, "                                                                                                                             ");
        file.write_all(format!("Row Count    : {}", row_count).as_bytes())
            .unwrap();
        file.write_all(format!("Column Count : {}", col_count).as_bytes())
            .unwrap();
        writeln!(file, "                                                                                                                             ");
        //file.write_all(format!("{:?}", table_structure).as_bytes()).unwrap();
        file.write_all(output.as_bytes()).unwrap();

        //println!("{}", output);
        let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
        let line_width =
            tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
        println!("Line width: {}", line_width);
        assert!(table_lines
            .iter()
            .all(|l| tabled::papergrid::util::string_width(l) == line_width));
        println!("Assert passed");
        println!("{}", "*".repeat(100))
    }

    #[quickcheck_macros::quickcheck]
    fn qc_test_data_refined(table_structure: TableStructure) {
        let rows = table_structure.rows;
        let theme = table_structure.theme;

        let mut builder = Builder::default();
        for r in rows.iter() {
            builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
        }

        let col_count = rows
            .iter()
            .map(|r| r.len())
            .reduce(usize::max)
            .unwrap_or_default();

        builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
        let mut table = builder.build();

        use ThemeFixture::*;
        match theme {
            Empty => {
                table.with(Style::empty());
            }
            Blank => {
                table.with(Style::blank());
            }
            Ascii => {
                table.with(Style::ascii());
            }
            Psql => {
                table.with(Style::psql());
            }
            Markdown => {
                table.with(Style::markdown());
            }
            Modern => {
                table.with(Style::modern());
            }
            Sharp => {
                table.with(Style::sharp());
            }
            Rounded => {
                table.with(Style::rounded());
            }
            Extended => {
                table.with(Style::extended());
            }
            Dots => {
                table.with(Style::dots());
            }
            RestructuredText => {
                table.with(Style::re_structured_text());
            }
            AsciiRounded => {
                table.with(Style::ascii_rounded());
            }
        }

        let output = table.to_string();

        let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
        let line_width =
            tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
        assert!(table_lines
            .iter()
            .all(|l| tabled::papergrid::util::string_width(l) == line_width));
    }

    #[test]
    fn test_data_span_test() {
        /*  let data1: Vec<Vec<u32>> = vec![
            vec![01, 02, 03, 04, 05],
            vec![11, 12, 13, 14, 15],
            vec![21, 22, 23, 24, 25],
            vec![31, 32, 33, 34, 35],
        ];

        let data2: Vec<Vec<u32>> = vec![
            vec![081, 082, 083],
            vec![811, 812, 813],
            vec![821, 822, 823],
            vec![831, 832, 833],
        ]; */

        /*    let data1: Vec<Vec<String>> = vec![
            vec!["01", "02", "03", "04", "05"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["11", "12", "13", "14", "15"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["21", "22", "23", "24", "25"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["31", "32", "33", "34", "35"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
        ];

        let data2: Vec<Vec<String>> = vec![
            vec!["081", "082", "083"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["811", "812", "813"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["821", "822", "823"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            vec!["831", "832", "833"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
        ]; */
        /* let table_structure = TableStructure {
            rows: data1,
            theme: ThemeFixture::Extended,
            row_span: vec![1, 2, 3],
            col_span: vec![3, 4, 5],
        }; */

        let table_structure = TableStructure {
            rows: vec![
                vec![
                    3090177132, 3404043224, 1564627439, 2958358353, 2078226218, 2314459898,
                    3296452771, 2940240882, 2405688409, 3434206839, 1672004660, 2993244949, 1,
                    2710954065, 1, 650630768, 4075780633, 3672961665, 875403794, 390684616,
                    2585537180, 1710161796, 0, 1782693506, 3179088110, 0, 3104854467, 453611743,
                    3904149215, 3078002024, 3884317813, 1164426068, 1211299999, 1090696884,
                    4159125006, 111875522, 3871591183, 3832736635, 2586380919, 140571445,
                    3871322366, 1443637070, 1285533426, 833063804, 4136332736, 2893527835,
                    3148407856, 2875582160, 1294559062, 738522469, 172505624, 642967718,
                    1848944128, 3295603117, 4112033834, 4183542208, 1465118059, 3152060171, 1,
                    599129407, 3059216985, 0, 2365777341, 560023850, 3967116013, 262642683,
                    1802329449, 1280070998, 4127126894, 1, 3666270608, 2862958730, 3560882178,
                ],
                vec![
                    944247898, 2745176781, 1, 3845662977, 693522648, 1869666159, 3581768726,
                ],
                vec![696555727, 1335494503, 2434205998, 3952900186, 455592141],
                vec![
                    3074431914, 4294967295, 3419631996, 1770208434, 3826391809, 561637605,
                    4294967295, 1923386823, 2023227658, 2557188742, 2974760393, 2485037303,
                    3803924911, 579515106, 140199194, 829834332, 1074880392, 412806750, 4294967295,
                    3673036455, 1934253816, 2856747097, 2368839900, 1062778343, 3675829300,
                    3642325457, 632508216, 1470007772, 1787828343, 2674792441, 0, 229494892,
                    2658104706, 4167937651, 3086879159, 1450071507, 1747776587, 3884794079,
                    4135074244, 1671086019, 775902798, 472844790, 280260773, 2514175821,
                    1703302970, 1071700265, 764875310, 127790992, 262057650, 744835384, 133060509,
                    663196857, 3977510543, 186118575, 1747936412, 2994425521, 2085770001,
                    2248641431, 2134721327, 1675446426, 2066693638, 0, 3600321562, 1917795145,
                    1595860780, 2947180967, 860880339, 1112984289, 916600990, 2603825803,
                    2699825004, 3462533094, 2400042941, 4174538129, 3523156061, 0, 284188272,
                    3352956589, 2587653220, 1607092233, 336992199, 804572855, 2237528624,
                    2345770428, 497587567, 574963334, 361951299, 4294967295, 3741743532,
                    3482755799,
                ],
                vec![
                    2697428679, 2057502563, 4252943750, 3877216908, 444022118, 397395413,
                    1214402862, 3113719310, 3627960565, 3369833241, 2174151783, 3523963507,
                    140047303, 1677452185, 1360845193, 3771852050, 426314995, 2184256720,
                    3671772729, 3899395260, 3765949426, 1518496958, 3590679611, 3611517548,
                    2063154848, 2954023869, 804126205, 2193511816, 1454032678, 2757961761,
                    1846664302, 2801911934, 2587741961,
                ],
                vec![
                    1701963585, 1590377138, 2925381013, 330400686, 4257786986, 3274640420,
                    709423553, 1, 1540174240, 1716656008, 1817146031, 2038364116, 648931263,
                    2661580165, 3884131805, 226240111, 4294967295, 1649659965, 1555561117,
                    2443295043, 1394566958, 1183339830, 397627246, 1019535650, 4107734945,
                    1336313390, 405994564, 3819440001, 3844277204, 845077155, 866612961,
                    4133979784, 4094892579, 230218114, 1751602344, 1802585412, 1650957848,
                    3867628593, 2259468542, 4294967295, 2107775994, 450603592, 1736896461,
                    3476824627, 2681936024, 1771273599, 1105124063, 4015559703, 1024453516,
                    1670741207, 3540863240, 2374982, 973540314, 1566017914, 598587941, 358188541,
                    1994771243, 516744413, 4039324942, 1292894630, 2212772545, 105531359,
                    3078548767, 4139192564, 3590336046, 1409248893, 2821008130, 2152020839,
                    4119645836, 3566242833, 1959999502, 3745272775, 723343621, 1816048681,
                    3072885351, 2586403451, 0, 1116394786, 3170411253, 823177850, 2545528567,
                    3486953277, 2468001272, 1684252092, 1243765723, 474970217,
                ],
                vec![
                    2196407376, 3315527889, 848898486, 1804077615, 3413535268, 2549190414,
                    2140335818, 1255109442, 4207056865, 4103432146, 1645488189, 1, 557180153,
                    2797980347, 1313325096, 683169194, 3089970409, 2718078782, 1164568484,
                    2366589847, 1280847701, 2387457556, 3422491828, 149303516, 4221892974,
                    1093378591, 1908790494, 2673083606, 3349684126, 2025523871, 3844058872,
                    1152466331, 3420667461, 2170483742, 2278882670, 2131172223, 2033445937,
                    1524745984, 35532092, 3526216715, 3568645812, 3447639647, 2880321296,
                    1007390680, 3640760297, 339207171, 4294967295, 1937873911, 2023106687,
                    2149319535, 1928140981, 2383931039, 1, 1993352207, 4188037947, 3572360443,
                    2201410855, 68424420, 4294967295, 101549948, 2931563463, 2706880226,
                    3509303792, 4251474878, 1094843821, 2204291959, 965246426, 3438141846,
                    136721328, 3346311090, 1302413227, 2242605401, 4271525516, 2553714679,
                    673101100, 1404330050, 1754565983, 186010142, 4221834004, 3028259420,
                    1528826779, 2283654490, 2841003803, 1488395958,
                ],
                vec![
                    1817066313, 236902451, 472734985, 3337676621, 227683287, 2499957015,
                    3875727034, 3131375735, 2527104087, 248536576, 1988352698, 2444432158,
                    1334149938, 4294967295, 4294967295, 1312643518, 1, 1840204292, 3392610108,
                    1188335110, 1004781625, 2265588952, 1941427610, 3105693056, 2562001582,
                    2146160205, 552524125, 1923211889, 879781321, 112237338, 2744994245,
                    2823829970, 2336943213, 4027111041, 4294967295, 947464520, 1438207887,
                    2498068523, 683511761, 3233412068, 1191172491,
                ],
                vec![
                    5040196, 1799163519, 359194625, 420872833, 952419482, 3305238156, 538618647, 0,
                    1207013135, 1692910443, 3716031014, 3114811122, 1406988562, 1367485518,
                    1447375408,
                ],
                vec![
                    1644225692, 2744289270, 129774325, 1765083256, 3067573644, 2845139625,
                    1183611998, 3210633903, 827608411, 3256702829, 2695122779, 4294967295,
                    1796227727, 4126088899, 254562290, 400132240, 463386264, 0, 3857872607,
                    43446198, 2286310760, 4019564085, 1971168724, 656474679, 1207197877,
                    4143068228, 1454120952, 738572178, 560067867, 3680790454, 1113356459,
                    3139452049, 4246735407, 1193553821, 2504142590, 907117000, 2127826334,
                    1333174485, 3178352788, 667520193, 2274973570, 2003842994, 2286816868,
                    1128497327, 272476914, 3043928251, 2209102446, 3909103253, 416052392, 1,
                    1409756502, 3095693937, 973998557, 1544301212, 4035395093, 2753847022,
                    4034559189, 3471305554, 4294967295, 2357085228, 0, 291167422, 2095853984,
                    4037688473, 4154178434, 4294967295, 2350392387, 985619289, 2993274737,
                    2316785839, 2986092315, 1679890376, 1806460955, 4294967295, 3902615446,
                    1298360903,
                ],
                vec![
                    271556827, 3018057050, 3649995335, 0, 3959084298, 2836638595, 3751426326,
                    1324791149, 1788196728, 3513135220, 4175788112, 699278795, 13212070,
                    3666546433, 307493025, 0, 0, 626294899, 140894954, 1, 3040325563, 1646560360,
                    1180312385, 2924658792, 845777678, 573208783, 1204672115, 2274776503,
                    1220567893, 4152172301, 2045890148, 4012368454, 372100889, 2349184390,
                    932032961, 278590088, 0, 4200974375, 2092114992, 3587529459, 3425585661,
                    4294178058, 1168329915, 1347504205, 586074916, 3097123061, 1654249759,
                    3955074724, 0, 3799079671, 380742363, 4138738739,
                ],
                vec![
                    3197254837, 2701212371, 0, 388810785, 2892682615, 174206783, 1068302082, 1,
                    139602685, 2459725374, 1586225329, 191357519, 4028042511, 3706237516,
                    2242562972, 1104807606, 1792724520, 2520530423, 0, 1005871724, 2039169156,
                    3444408458, 3600230203, 0, 3129371475, 2218260300, 3875788468, 3027956084, 0,
                    0, 3723745872, 1152406569, 622760692, 1687656003, 1, 3136172333, 178175030,
                    2924328637, 811024834, 2496671685, 4287642633, 2155195664, 655741057,
                ],
                vec![
                    971122847, 3445033352, 2521715193, 1655283919, 3905717627, 162183887,
                    1055867840, 3072568871, 3536830405, 3166790986, 153981178, 2510273172,
                    1824818397, 2079924400, 3677668655, 205751550, 2922414865, 0, 1, 1312348628,
                    287860374, 3890132039, 2485035207, 1096184530, 2606474377, 260981902,
                    2129036469, 150875705, 3775227412, 1141090149, 1669110661, 1059388603,
                    2777727167,
                ],
                vec![
                    1414742400, 2690109551, 2795120949, 1466225467, 3855951765, 0, 2234651771,
                    1090173929, 3485913772, 3313994629, 3150321056, 150426209, 1716309272,
                    1638308249, 2977034366, 1038102712, 3666016244, 820434834, 1611331051,
                    2527166315, 3242795233, 2511949954, 3956337182, 3549933201, 3314552821,
                    3077940144, 3322845205, 2651483956, 2509730666, 1715770194, 3649705027, 0,
                    1542742300, 555328273, 3994062606, 3776451736, 2213572954, 4176674490,
                    2170446520, 3362231453, 3106395435, 1694961435, 2184013207, 3474421640,
                    1160497507, 431238770, 3183511179, 1399327567, 4294967295, 851419872,
                    1360253199, 1217670329, 1641821493, 3232719245, 1, 3837989818, 84540564,
                    4023205109, 1829323388, 2582905370, 3102695986, 3660004174, 4294967295,
                    27023999, 3966851059, 4086060844, 2500845049, 2035295513, 3483962796,
                    1041217335, 254878349, 359415905, 1338517029, 2534539568, 743495923,
                    3850567459, 892542993, 154083827, 1694287149, 236824752, 1377347212, 4980235,
                    3014784389, 3050415587, 2629359798, 4214752373, 2254267102, 4095296254,
                    2536934931, 2720079412, 2107685468, 4074518490,
                ],
                vec![
                    1795042970, 2184835732, 2565161959, 3351343722, 714553431, 105831373,
                    855995526, 466583473, 3882293004, 4294967295, 0, 3632581254,
                ],
                vec![
                    3816356986, 1, 1487846853, 64021843, 74839385, 762947444, 2493809313,
                    724900875, 45493832, 2559540153, 985813623, 3072706147, 2832922799, 301881637,
                    1465045799, 1, 2620524248, 1939820031, 1208439495, 575205615, 2306677296,
                    1918271376, 3480781354,
                ],
                vec![
                    3026727459, 889339439, 1309946283, 1992697613, 1204889133, 2960237876,
                    2789850350, 2596428772, 751123947, 919694929, 0, 4292601550, 0, 3024188995,
                    2015140687, 0, 1860652117, 1095623645, 0, 1775448658, 1835517439, 0,
                    2813295052, 2836406675, 3805695089, 3208158671, 1876001269, 2768203215, 1,
                    1126725344, 1530611499, 337440504, 239401505, 2466964589, 1769278956,
                    372532606, 85406003, 4135956081, 3203961228, 1535241928, 1636966900,
                    1417958523, 1808271984, 2255761097, 0, 0, 144531296, 1504525098, 2541276906,
                    2163397542, 0, 397878524, 1739470414, 4244594041, 3930845731, 2804338480,
                    3147896324, 3618066972, 3336577388, 84594849, 3225302340, 987539640,
                    2734696435, 4172199301, 3145933866, 1676704138, 0, 3797065235, 797322797,
                    1103284198, 1415556807, 1008515285, 4021731247, 740268910, 3722499345,
                    460858037, 284306775, 2593097596, 2171909922, 553539953, 2814257308, 767170632,
                    3131938624, 3189794233, 0, 4134851989,
                ],
                vec![1249083565, 3797584783],
                vec![
                    71667426, 3420822447, 545060344, 646165524, 1287040376, 538581017, 1,
                    996720001, 2525031951, 87049440, 4227589316, 1424847814, 4177981817,
                    2974385217, 2748815409, 3637682033, 2125810075, 3957246128, 2826606811,
                    4294967295, 3479744905, 1918513646, 3054229087, 2163020992, 394613382,
                    1033368881, 3759259057, 3558741922, 822232023, 3865897223, 4071485491,
                    1030334702, 2904620434, 0, 187140281, 2404445197, 4133006815, 3964173324, 0, 0,
                    3027612022, 858649719, 2583594183, 3069264180, 3697253036, 2838520590,
                    4048836918, 2275782286, 2488259908, 894154913, 4051613150, 1397406454,
                    2711678711, 2129814351, 2915167590, 671465218, 276945729, 1468102530,
                    3240301053, 1911648628, 711185215, 2913864444, 2823340984, 4202237486,
                    2802968008, 1406159322, 3404278973, 2013645719, 4068394505, 1868436759,
                    387071214, 107758737, 4294967295, 4294967295, 1807568427, 920774379,
                    2892163064, 1443080164, 1274661752, 1951801711, 3873397616, 4261984362,
                    247412424, 3835819599,
                ],
            ],
            theme: Modern,
            row_span: vec![0, 2],
            col_span: vec![10, 3],
        };

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .read(true)
            .open("table.txt")
            .unwrap();

        let rows = table_structure.clone().rows;
        let theme = table_structure.clone().theme;
        let row_count = rows.len();
        let col_count = rows
            .iter()
            .map(|r| r.len())
            .reduce(usize::max)
            .unwrap_or_default();

        println!("Rows Count      : {}", row_count);
        println!("Column Count    : {}", col_count);

        let mut row_span = table_structure.row_span;
        let mut col_span = table_structure.col_span;

        let mut builder = Builder::default();
        for r in rows.iter() {
            builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
        }

        builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
        let mut table = builder.build();

        for rr in 1..row_count {
            if row_span.is_empty() {
                break;
            }
            for cc in 1..col_count {
                if row_span.is_empty() || col_span.is_empty() {
                    break;
                }
                println!(
                    "rr: {}, cc: {}, row_span: {:?}, col_span: {:?}",
                    rr, cc, row_span, col_span
                );
                table.with(
                    Modify::new(Cell(rr, cc))
                        .with(Span::row(row_span.pop().unwrap_or(1) as usize))
                        .with(Span::column(col_span.pop().unwrap_or(1) as usize)),
                );
            }
        }

        /* table.with(
            Modify::new(Cell(1, 0))
                .with(Span::row(2))
                .with(Span::column(5)),
        ); */
        use ThemeFixture::*;
        match theme {
            Empty => {
                table.with(Style::empty().vertical('|'));
            }
            Blank => {
                table.with(Style::blank().vertical('|'));
            }
            Ascii => {
                table.with(Style::ascii());
            }
            Psql => {
                table.with(Style::psql());
            }
            Markdown => {
                table.with(Style::markdown());
            }
            Modern => {
                table.with(Style::modern().vertical('|'));
            }
            Sharp => {
                table.with(Style::sharp());
            }
            Rounded => {
                table.with(Style::rounded());
            }
            Extended => {
                table.with(Style::extended());
            }
            Dots => {
                table.with(Style::dots());
            }
            RestructuredText => {
                table.with(Style::re_structured_text());
            }
            AsciiRounded => {
                table.with(Style::ascii_rounded());
            }
        }

        let output = table.to_string();
        //println!("{:?}", table_structure);

        writeln!(file, "                                                                                                                             ");
        writeln!(file, "                                                                                                                             ");
        file.write_all(format!("Row Count    : {}", row_count).as_bytes())
            .unwrap();
        file.write_all(format!("Column Count : {}", col_count).as_bytes())
            .unwrap();
        writeln!(file, "                                                                                                                             ");
        //file.write_all(format!("{:?}", table_structure).as_bytes()).unwrap();
        file.write_all(output.as_bytes()).unwrap();

        //println!("{}", output);
        let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
        let line_width =
            tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
        println!("Line width: {}", line_width);
        table_lines
            .iter()
            .for_each(|l| println!("LW: {}", tabled::papergrid::util::string_width(l)));

        assert!(table_lines
            .iter()
            .all(|l| tabled::papergrid::util::string_width(l) == line_width));
        println!("Assert passed");
        println!("{}", "*".repeat(100))
    }

    #[quickcheck_macros::quickcheck]
    fn qc_test_data_span_test(table_structure: TableStructure) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .read(true)
            .open("table.txt")
            .unwrap();

        let rows = table_structure.clone().rows;
        let theme = table_structure.clone().theme;
        let row_count = rows.len();
        let col_count = rows
            .iter()
            .map(|r| r.len())
            .reduce(usize::max)
            .unwrap_or_default();

        println!("Rows Count      : {}", row_count);
        println!("Column Count    : {}", col_count);

        let mut row_span = table_structure.row_span;
        let mut col_span = table_structure.col_span;

        let mut builder = Builder::default();
        for r in rows.iter() {
            builder.add_record(r.iter().map(|e| e.to_string()).collect::<Vec<String>>());
        }

        builder.set_columns((1..col_count + 1).map(|head| head.to_string()));
        let mut table = builder.build();

        for rr in 1..row_count {
            if row_span.is_empty() && col_span.is_empty() {
                break;
            }
            for cc in 1..col_count {
                println!(
                    "rr: {}, cc: {}, row_span: {:?}, col_span: {:?}",
                    rr, cc, row_span, col_span
                );
                table.with(
                    Modify::new(Cell(rr, cc))
                        .with(Span::row(row_span.pop().unwrap_or(1) as usize))
                        .with(Span::column(col_span.pop().unwrap_or(1) as usize)),
                );
            }
        }

        /* table.with(
            Modify::new(Cell(1, 0))
                .with(Span::row(2))
                .with(Span::column(5)),
        ); */
        use ThemeFixture::*;
        match theme {
            Empty => {
                table.with(Style::empty());
            }
            Blank => {
                table.with(Style::blank());
            }
            Ascii => {
                table.with(Style::ascii());
            }
            Psql => {
                table.with(Style::psql());
            }
            Markdown => {
                table.with(Style::markdown());
            }
            Modern => {
                table.with(Style::modern());
            }
            Sharp => {
                table.with(Style::sharp());
            }
            Rounded => {
                table.with(Style::rounded());
            }
            Extended => {
                table.with(Style::extended());
            }
            Dots => {
                table.with(Style::dots());
            }
            RestructuredText => {
                table.with(Style::re_structured_text());
            }
            AsciiRounded => {
                table.with(Style::ascii_rounded());
            }
        }

        let output = table.to_string();
        //println!("{:?}", table_structure);

        writeln!(file, "                                                                                                                             ");
        writeln!(file, "                                                                                                                             ");
        file.write_all(format!("Row Count    : {}", row_count).as_bytes())
            .unwrap();
        file.write_all(format!("Column Count : {}", col_count).as_bytes())
            .unwrap();
        writeln!(file, "                                                                                                                             ");
        //file.write_all(format!("{:?}", table_structure).as_bytes()).unwrap();
        file.write_all(output.as_bytes()).unwrap();

        //println!("{}", output);
        let table_lines: Vec<String> = output.lines().map(|l| l.into()).collect();
        let line_width =
            tabled::papergrid::util::string_width(table_lines.first().unwrap_or(&"".to_owned()));
        println!("Line width: {}", line_width);
        assert!(table_lines
            .iter()
            .all(|l| tabled::papergrid::util::string_width(l) == line_width));
        println!("Assert passed");
        println!("{}", "*".repeat(100))
    }
}
