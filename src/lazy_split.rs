struct LazySplit<I, P> {
    iter: I,
    predicate: P,
    flag: bool,
}

impl<I, P> LazySplit<I, P> {
    fn new(iter: I, predicate: P) -> LazySplit<I, P> {
        LazySplit {
            iter,
            predicate,
            flag: false,
        }
    }
}

impl<I: Iterator, P> Iterator for LazySplit<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = SubLazySplit<I, P>;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.iter.next();
        if sample.is_some() {
            Some(SubLazySplit {
                iter: self.iter,
                sample,
                predicate: self.predicate,
            })
        } else {
            None
        }
    }
}
struct SubLazySplit<I: Iterator, P> {
    iter: I,
    predicate: P,
    sample: Option<I::Item>,
}
impl<I: Iterator, P> Iterator for SubLazySplit<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(exists) = self.sample {
            let exists = exists;
            // pop the sample
            self.sample = None;
            Some(exists)
        } else {
            self.iter.next()
        }
    }
}
