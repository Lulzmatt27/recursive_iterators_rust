#[derive(Debug)]
struct Node{
    values: Vec<i32>,
    children: Vec<Node>
}

struct NodeIter<'a>{
    viter: Box<Iterator<Item=&'a i32> + 'a>,
    citer: Box<Iterator<Item=&'a Node> + 'a>,
}

impl <'a> Iterator for NodeIter<'a>{
    type Item = & 'a i32;
    fn next(&mut self)->Option<Self::Item>{
        if let Some(val) = self.viter.next(){
            println!("{:?} val", val);
            Some(val)
        } else{
            if let Some(child) = self.citer.next(){
                println!("child {:?}", child);
                self.viter = Box::new(child.values());
                self.next()
            }
            else{
                None
            }
        }
    }
}

impl Node{
    // fn values<'a>(&'a self) -> NodeIter<'a>{
    //    NodeIter{
    //        viter: Box::new(self.values.iter()),
    //        citer: Box::new(self.children.iter()),
    //    }
    // }

    pub fn values<'a>(&'a self) -> Box<Iterator<Item = &i32>+ 'a > {
        Box::new(
            self.values
                .iter()
                .chain(self.children.iter().map(|n| n.values()).flatten()),
        )
    }
}



fn main() {
    let n = Node {
        values: vec![1,2,3],
        children: vec![
            Node{
                values: vec![4,5,6],
                children: vec![]
            },
            Node{
                values: vec![7,8,9],
                children: vec![]
            }
        ]
    };

    let v: Vec<_> = n.values().collect();
    println!("flatt vec {:?}", v);
}
