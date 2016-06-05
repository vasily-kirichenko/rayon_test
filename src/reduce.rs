// let reduceParallelHopac<'a> f (a: 'a array) = 
//     let rec reduceRec (f, ie: 'a array) = 
//         match ie.Length with
//         | 1 -> Job.result ie.[0]
//         | 2 -> Job.result (f ie.[0] ie.[1])
//         | len -> 
//             let h = len / 2
//             reduceRec (f, ie.[0..h - 1]) <*> Job.delayWith reduceRec (f, ie.[h..]) |>> fun (x, y) -> f x y
//     match a.Length with
//     | 0 -> failwith "Sequence contains no elements"
//     | _ -> run <| reduceRec (f, a)