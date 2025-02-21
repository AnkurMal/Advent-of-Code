const DATA: &str = include_str!("../../data/year2024/day_12.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Search {
    Done(char),
    NotDone(char)
}

pub fn part_1() {
    let (mut grid, width, height) = process_data();
    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            if let Search::NotDone(ch) = grid[i][j] {
                let mut rec = vec![(i, j)];
                let (mut perimeter, mut area) = (0, 0);

                while let Some((y, x)) = rec.pop() {
                    grid[y][x] = Search::Done(ch);
                    area += 1;

                    if y!=0 {
                        let curr_ch = grid[y-1][x];
                        let new_coord = (y-1, x);

                        if curr_ch==Search::NotDone(ch) { if !rec.contains(&new_coord) {rec.push(new_coord); }}
                        else if let Search::Done(char) = curr_ch {
                            if char!=ch { perimeter += 1 }
                        } else { perimeter += 1 }
                    } else { perimeter += 1 }

                    if x!=0 {
                        let curr_ch = grid[y][x-1];
                        let new_coord = (y, x-1);

                        if curr_ch==Search::NotDone(ch) { if !rec.contains(&new_coord) {rec.push(new_coord); }}
                        else if let Search::Done(char) = curr_ch {
                            if char!=ch { perimeter += 1 }
                        }else { perimeter += 1 }
                    } else { perimeter += 1 }
                    
                    if y!=height-1 {
                        let curr_ch = grid[y+1][x];
                        let new_coord = (y+1, x);

                        if curr_ch==Search::NotDone(ch) { if !rec.contains(&new_coord) {rec.push(new_coord); }}
                        else if let Search::Done(char) = curr_ch {
                            if char!=ch { perimeter += 1 }
                        }else { perimeter += 1 }
                    } else { perimeter += 1 }

                    if x!=width-1 {
                        let curr_ch = grid[y][x+1];
                        let new_coord = (y, x+1);

                        if curr_ch==Search::NotDone(ch) { if !rec.contains(&new_coord) {rec.push(new_coord); }}
                        else if let Search::Done(char) = curr_ch {
                            if char!=ch { perimeter += 1 }
                        }else { perimeter += 1 }
                    } else { perimeter += 1 }
                }
                sum += area*perimeter;
            }
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let (mut grid, width, height) = process_data();
    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            if let Search::NotDone(ch) = grid[i][j] {
                let mut rec = vec![(i, j)];
                let mut area = 0;
                let mut corners = 0;

                while let Some((y, x)) = rec.pop() {
                    grid[y][x] = Search::Done(ch);
                    area += 1;

                    if ((y<height-1 && x<width-1) && 
                    (Search::Done(ch)==grid[y][x+1] || Search::NotDone(ch)==grid[y][x+1]) && 
                    (Search::Done(ch)==grid[y+1][x] || Search::NotDone(ch)==grid[y+1][x]) &&
                    (y==0 || (Search::Done(ch)!=grid[y-1][x] && Search::NotDone(ch)!=grid[y-1][x])) &&
                    (x==0 || (Search::Done(ch)!=grid[y][x-1] && Search::NotDone(ch)!=grid[y][x-1]))) ||

                    ((y<height-1 && x>0) && 
                    (Search::Done(ch)==grid[y][x-1] || Search::NotDone(ch)==grid[y][x-1]) && 
                    (Search::Done(ch)==grid[y+1][x] || Search::NotDone(ch)==grid[y+1][x]) &&
                    (y==0 || (Search::Done(ch)!=grid[y-1][x] && Search::NotDone(ch)!=grid[y-1][x])) &&
                    (x==width-1 || (Search::Done(ch)!=grid[y][x+1] && Search::NotDone(ch)!=grid[y][x+1]))) ||

                    ((y>0 && x>0) && 
                    (Search::Done(ch)==grid[y][x-1] || Search::NotDone(ch)==grid[y][x-1]) && 
                    (Search::Done(ch)==grid[y-1][x] || Search::NotDone(ch)==grid[y-1][x]) &&
                    (y==height-1 || (Search::Done(ch)!=grid[y+1][x] && Search::NotDone(ch)!=grid[y+1][x])) &&
                    (x==width-1 || (Search::Done(ch)!=grid[y][x+1] && Search::NotDone(ch)!=grid[y][x+1]))) ||

                    ((y>0 && x<width-1) && 
                    (Search::Done(ch)==grid[y][x+1] || Search::NotDone(ch)==grid[y][x+1]) && 
                    (Search::Done(ch)==grid[y-1][x] || Search::NotDone(ch)==grid[y-1][x]) &&
                    (y==height-1 || (Search::Done(ch)!=grid[y+1][x] && Search::NotDone(ch)!=grid[y+1][x])) &&
                    (x==0 || (Search::Done(ch)!=grid[y][x-1] && Search::NotDone(ch)!=grid[y][x-1]))) 
                    {
                        corners += 1;
                    }

                    if ((y<height-1 && x<width-1) && 
                    (Search::Done(ch)==grid[y][x+1] || Search::NotDone(ch)==grid[y][x+1]) && 
                    (Search::Done(ch)==grid[y+1][x] || Search::NotDone(ch)==grid[y+1][x]) &&
                    (Search::Done(ch)!=grid[y+1][x+1] && Search::NotDone(ch)!=grid[y+1][x+1]))
                    {
                        corners += 1;
                    }

                    if ((y<height-1 && x>0) && 
                    (Search::Done(ch)==grid[y][x-1] || Search::NotDone(ch)==grid[y][x-1]) && 
                    (Search::Done(ch)==grid[y+1][x] || Search::NotDone(ch)==grid[y+1][x]) &&
                    (Search::Done(ch)!=grid[y+1][x-1] && Search::NotDone(ch)!=grid[y+1][x-1])) 
                    {
                        corners += 1;
                    }

                    if ((y>0 && x>0) && 
                    (Search::Done(ch)==grid[y][x-1] || Search::NotDone(ch)==grid[y][x-1]) && 
                    (Search::Done(ch)==grid[y-1][x] || Search::NotDone(ch)==grid[y-1][x]) &&
                    (Search::Done(ch)!=grid[y-1][x-1] && Search::NotDone(ch)!=grid[y-1][x-1])) 
                    {
                        corners += 1;
                    }

                    if ((y>0 && x<width-1) && 
                    (Search::Done(ch)==grid[y][x+1] || Search::NotDone(ch)==grid[y][x+1]) && 
                    (Search::Done(ch)==grid[y-1][x] || Search::NotDone(ch)==grid[y-1][x]) &&
                    (Search::Done(ch)!=grid[y-1][x+1] && Search::NotDone(ch)!=grid[y-1][x+1]))
                    {
                        corners += 1;
                    }

                    if (y<height-1 && (Search::Done(ch)==grid[y+1][x] || Search::NotDone(ch)==grid[y+1][x]) &&
                    (y==0 || (Search::Done(ch)!=grid[y-1][x] && Search::NotDone(ch)!=grid[y-1][x])) &&
                    (x==0 || (Search::Done(ch)!=grid[y][x-1] && Search::NotDone(ch)!=grid[y][x-1])) &&
                    (x==width-1 || (Search::Done(ch)!=grid[y][x+1] && Search::NotDone(ch)!=grid[y][x+1]))) ||

                    (y>0 && (Search::Done(ch)==grid[y-1][x] || Search::NotDone(ch)==grid[y-1][x]) &&
                    (y==height-1 || (Search::Done(ch)!=grid[y+1][x] && Search::NotDone(ch)!=grid[y+1][x])) &&
                    (x==0 || (Search::Done(ch)!=grid[y][x-1] && Search::NotDone(ch)!=grid[y][x-1])) &&
                    (x==width-1 || (Search::Done(ch)!=grid[y][x+1] && Search::NotDone(ch)!=grid[y][x+1]))) 
                    {
                        corners += 2;
                    }

                    if (x<width-1 && (Search::Done(ch)==grid[y][x+1] || Search::NotDone(ch)==grid[y][x+1]) &&
                    (y==0 || (Search::Done(ch)!=grid[y-1][x] && Search::NotDone(ch)!=grid[y-1][x])) &&
                    (x==0 || (Search::Done(ch)!=grid[y][x-1] && Search::NotDone(ch)!=grid[y][x-1])) &&
                    (y==height-1 || (Search::Done(ch)!=grid[y+1][x] && Search::NotDone(ch)!=grid[y+1][x]))) ||

                    (x>0 && (Search::Done(ch)==grid[y][x-1] || Search::NotDone(ch)==grid[y][x-1]) &&
                    (y==height-1 || (Search::Done(ch)!=grid[y+1][x] && Search::NotDone(ch)!=grid[y+1][x])) &&
                    (y==0 || (Search::Done(ch)!=grid[y-1][x] && Search::NotDone(ch)!=grid[y-1][x])) &&
                    (x==width-1 || (Search::Done(ch)!=grid[y][x+1] && Search::NotDone(ch)!=grid[y][x+1])))
                    {
                        corners += 2;
                    }

                    if y!=0 {
                        let curr_ch = grid[y-1][x];
                        let new_coord = (y-1, x);
                        if curr_ch == Search::NotDone(ch) && !rec.contains(&new_coord) {rec.push(new_coord); }
                    }

                    if x!=0 {
                        let curr_ch = grid[y][x-1];
                        let new_coord = (y, x-1);
                        if curr_ch == Search::NotDone(ch) && !rec.contains(&new_coord) { rec.push(new_coord); }
                    } 
                    
                    if y!=height-1 {
                        let curr_ch = grid[y+1][x];
                        let new_coord = (y+1, x);
                        if curr_ch == Search::NotDone(ch) && !rec.contains(&new_coord) {rec.push(new_coord); }
                    } 

                    if x!=width-1 {
                        let curr_ch = grid[y][x+1];
                        let new_coord = (y, x+1);
                        if curr_ch == Search::NotDone(ch) && !rec.contains(&new_coord) {rec.push(new_coord); }
                    }
                }
                
                if area==1 {
                    corners = 4;
                }
                sum += area*corners;
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn process_data() -> (Vec<Vec<Search>>, usize, usize) {

    let grid = DATA
    .lines()
    .map(|x| x.chars().map(Search::NotDone).collect::<Vec<Search>>())
    .collect::<Vec<Vec<Search>>>();

    let width = grid[0].len();
    let height = grid.len();
    
    (grid, width, height)
}