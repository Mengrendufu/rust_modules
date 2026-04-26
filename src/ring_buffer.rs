//============================================================================
// Copyright (C) 2026 Sunny Matato
//
// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar.
// See http://www.wtfpl.net/ for more details.
//============================================================================
pub struct RingBuffer<T, const N: usize> {
    sto: [Option<T>; N],
    qhead: usize,
    qtail: usize,
    nUsed: usize,
    nMin: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            sto: core::array::from_fn(|_| None),
            qhead: 0,
            qtail: 0,
            nUsed: 0,
            nMin: N,
        }
    }

    pub fn isEmpty(&self) -> bool {
        self.nUsed == 0
    }

    pub fn isFull(&self) -> bool {
        self.nUsed >= N
    }

    pub fn len(&self) -> usize {
        self.nUsed
    }

    pub fn minFree(&self) -> usize {
        self.nMin
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.isFull() {
            Err(item)
        } else {
            self.sto[self.qhead] = Some(item);
            if self.qhead == 0 {
                self.qhead = N;
            }
            self.qhead -= 1;
            self.nUsed += 1;
            if self.nMin > N - self.nUsed {
                self.nMin = N - self.nUsed;
            }
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.isEmpty() {
            None
        } else {
            let popItem = self.sto[self.qtail].take();
            if self.qtail == 0 {
                self.qtail = N;
            }
            self.qtail -= 1;
            self.nUsed -= 1;
            popItem
        }
    }
}
