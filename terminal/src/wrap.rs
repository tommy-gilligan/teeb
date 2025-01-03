// struct WrappedLines<'a, const COLS: usize> (
//     OldestOrdered<'a, char, 5000>,
//     Vec<char, COLS>,
//     usize
// );
//
// #[derive(PartialEq)]
// enum WrappedLine<const COLS: usize> {
//     Accumulating,
//     Line(Vec<char, COLS>)
// }
//
// impl <'a, const COLS: usize>WrappedLines<'a, COLS> {
//     fn new(characters: OldestOrdered<'a, char, 5000>) -> Self {
//         Self(characters, Vec::new(), 0)
//     }
// }
//
// impl <'a, const COLS: usize>Iterator for WrappedLines<'a, COLS> {
//     type Item = WrappedLine::<COLS>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.0.next() {
//             None => None,
//             Some('\n' | '\r') => {
//                 self.2 = 0;
//                 let mut result = Vec::new();
//                 mem::swap(&mut result, &mut self.1);
//                 Some(WrappedLine::Line(result))
//             },
//             Some(c) => {
//                 if self.2 >= COLS {
//                     self.2 = 0;
//                     let mut result = Vec::new();
//                     mem::swap(&mut result, &mut self.1);
//                     let _ = self.1.push(*c);
//                     Some(WrappedLine::Line(result))
//                 } else {
//                     self.2 += 1;
//                     let _ = self.1.push(*c);
//                     Some(WrappedLine::Accumulating)
//                 }
//             }
//         }
//     }
// }
