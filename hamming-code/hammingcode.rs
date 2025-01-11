use std::io::{self, BufReader, BufRead};
 
fn ham_encode() -> io::Result<()> {
      let file = std::fs::File::open("Encoding.txt")?;
      let reader = BufReader::new(file);
  
      for line in reader.lines() {
          let cur_line = line?;
          let mut hamm_list: Vec<String> = vec![];
          let mut tok_list: Vec<i32> = vec![];
          
          println!("Processing: {}", cur_line); // each line
          for x in cur_line.chars() { // each char of each line
              let chr = x as u8;
              let chr_bits = format!("{:07b}", chr);
              let mut bit_vec = vec![];
  
              for b in chr_bits.chars() {
                  let bit = b.to_string();
                  bit_vec.push(bit);
              }
  
              bit_vec.insert(3, 9.to_string()); // pos, element
              bit_vec.insert(7, 8.to_string()); // pos, element
              bit_vec.insert(9, 7.to_string()); // pos, element
              bit_vec.insert(10, 6.to_string()); // pos, element
  
              // calc p1 bit
              let mut num_ones = 0;
              for ab in 0..11 {
                  if ab == 0 || ab == 2 || ab == 4 || ab == 6 || ab == 8 
                  {
                      if bit_vec[ab] == "1" {num_ones += 1}
                  }
              } if num_ones % 2 == 0 {bit_vec[10] = 0.to_string();} else {bit_vec[10] = 1.to_string();}
  
              // calc p2 bit
              num_ones = 0;
              for ab in 0..11 {
                  if ab == 0 || ab == 1 || ab == 4 || ab == 5 || ab == 8 
                  {
                      if bit_vec[ab] == "1" {num_ones += 1}
                  }
              } if num_ones % 2 == 0 {bit_vec[9] = 0.to_string();} else {bit_vec[9] = 1.to_string();}
  
             // calc p3 bit
              num_ones = 0;
              for ab in 0..11 {
                  if ab == 4 || ab == 5 || ab == 6
                  {
                      if bit_vec[ab] == "1" {num_ones += 1}
                  }
              } if num_ones % 2 == 0 {bit_vec[7] = 0.to_string();} else {bit_vec[7] = 1.to_string();}
  
             // calc p4 bit
              num_ones = 0;
              for ab in 0..11 {
                  if ab == 0 || ab == 1 || ab == 2
                  {
                      if bit_vec[ab] == "1" {num_ones += 1}
                  }
              } if num_ones % 2 == 0 {bit_vec[3] = 0.to_string();} else {bit_vec[3] = 1.to_string();}
  
             let cool = bit_vec.join(""); // hmm
             let mut notcool = String::from("00000");
             notcool.push_str(&cool);
             let tok = i32::from_str_radix(&cool, 2).unwrap(); // hmm
 
             println!("Character = {}, Token = {}", x, tok);
             hamm_list.push(notcool);
             tok_list.push(tok);
          }
 
          println!("Phrase: {}", cur_line);
          println!("Hamming Code: ");
             let mut count = 0;
             let temp = tok_list.len();
             let mut f = 0;
             while f < temp {
                 print!("{} ({}) ", tok_list[f], hamm_list[f]); count += 1;
                 if count % 3 == 0 {print!("\n");}
                 f += 1;
             } println!("\n");
 
      } Ok(())
  }
  
fn ham_decode() -> io::Result<()> {
    let file = std::fs::File::open("Decoding.txt")?;
    let reader = BufReader::new(file);
    let mut block_num = 1;
    let mut line_num = 1;
    let mut char_vec: Vec<char> = Vec::new();

    for line in reader.lines() {
       let cur_line = line?;
       let token = cur_line.split_whitespace();

       // if "=" do some stuff
       if cur_line.len() < 3 {
           print!("\n");
           line_num = 1;
           let result_str: String = char_vec.iter().collect();
           println!(">>> Decoded message ({}/5): {}\n", block_num, result_str);
           block_num +=1;
           char_vec.clear();
       }
       else {
       println!("Processing block [{}], line [{}]:", block_num, line_num);

       // for each number
       for xx in token {
           let num: u32 = xx.parse().expect("");
           let tok_bits = format!("{:016b}", num);
           let mut bin_str = String::new();
           let mut index = 0;
           let mut par_bits = String::new();
           let mut par_bits2 = String::new();

           for s in tok_bits.chars() {
              if index > 4 {
               if index != 8 && index != 12 && index != 14 && index != 15 {
                   bin_str.push(s);
               } 
               else {par_bits.push(s)}
              } index += 1;
           }
          
           let old_bin = bin_str.clone();
           bin_str.insert(3, 'x'); bin_str.insert(7, 'x'); bin_str.insert(9, 'x'); bin_str.insert(10, 'x');

           let bin_str_chars: Vec<char> = bin_str.chars().collect();

          // calc par bit 1
           let mut num_ones = 0;
            for ab in 0..11 {
                if ab == 0 || ab == 2 || ab == 4 || ab == 6 || ab == 8 
                {
                    if bin_str_chars[ab] == '1' {num_ones += 1}
                }
            } if num_ones % 2 == 0 {par_bits2.insert(0, '0');} else {par_bits2.insert(0,'1');}

            // calc par bit 2
            num_ones = 0;
            for ab in 0..11 {
              if ab == 0 || ab == 1 || ab == 4 || ab == 5 || ab == 8 
              {
                  if bin_str_chars[ab] == '1' {num_ones += 1}
              }
          } if num_ones % 2 == 0 {par_bits2.insert(0,'0');} else {par_bits2.insert(0,'1');}

          // bit 3
          num_ones = 0;
          for ab in 0..11 {
              if ab == 4 || ab == 5 || ab == 6 
              {
                  if bin_str_chars[ab] == '1' {num_ones += 1}
              }
          } if num_ones % 2 == 0 {par_bits2.insert(0,'0');} else {par_bits2.insert(0,'1');}

          // par bit 4
          num_ones = 0;
          for ab in 0..11 {
              if ab == 0 || ab == 1 || ab == 2 
              {
                  if bin_str_chars[ab] == '1' {num_ones += 1}
              }
          } if num_ones % 2 == 0 {par_bits2.insert(0,'0');} else {par_bits2.insert(0,'1');}
       
           let deci = u32::from_str_radix(&old_bin, 2).expect("");
           let ascii = char::from_u32(deci).expect("");
           char_vec.push(ascii);

           // error detection
           if par_bits == par_bits2 {
           println!("   No errors found in: {}, {} = '{}'", xx, tok_bits, ascii);
           } else {
              println!("   Error found in: {}, {} = '{}'", xx, tok_bits, ascii); 
           }
           
       } line_num += 1;
   }
   } Ok(())
}
  
fn main() -> io::Result<()> {
  
  println!("\nEncoding");
  println!("-------------------------------------------------------------");
  ham_encode()?;
  
  println!("Decoding");
  println!("-------------------------------------------------------------");
  ham_decode()?;
  println!("\n");
  
  Ok(())
} 
