#[cfg(test)]
mod tests {

    use quickcheck::Arbitrary;
    use tabled::{builder::Builder, Style};

    #[derive(Clone, Debug)]
    struct TableStructure {
        pub rows: Vec<Line>,
        pub theme: ThemeFixture,
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
}
