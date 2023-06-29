pub use self::robinson::DistanceMatrice;
pub use self::robinson::Robin;
pub mod robinson {

    //! Robinson modules
    //!
    //! This crate is a direct implementation of the algorithms presented in "MODULES IN ROBINSON SPACES" by MIKHAEL  CARMONA,  VICTOR  CHEPOI,  GUYSLAIN  NAVES AND PASCAL  PREA.
    //!
    //! It can determine if a given square matrice admit a compatible order in O(n²). If it does admit at least one, this crate will provide you with a permutation corresponding to a compatible order.

    mod func;

    use func::{get_empty_distance_matrice, max};

    use avl::AvlTreeMap;

    ///This structure will be able to determine if its given `DistanceMatrice` admit a compatible order (using .resolve_robin) and provide you with one (in solved_permut).
    pub struct Robin {
        pub dist: DistanceMatrice,
        pub solved_permut: Option<Vec<u32>>,
    }
    impl Robin {
        ///Create a Robin object
        ///
        /// Needs a square matrice as argument
        ///
        /// Panic :
        ///
        /// This function will panic if provided with a non square matrice
        pub fn new(distance_mat: Vec<Vec<u32>>) -> Robin {
            let n = distance_mat.len();
            for line in &distance_mat {
                if line.len() != n {
                    panic!("Given distance matrice isn't square");
                }
            }
            return Robin {
                dist: DistanceMatrice { distance_mat },
                solved_permut: None,
            };
        }

        fn refine(&self, q: u32, s_list: Vec<u32>) -> Vec<Vec<u32>> {
            let mut tree: AvlTreeMap<u32, Vec<u32>> = AvlTreeMap::new();
            for elem in s_list {
                match tree.get(&self.dist.d(q, elem)) {
                    Some(l) => {
                        tree.insert(self.dist.d(q, elem), [vec![elem], l.clone()].concat());
                    }
                    None => {
                        tree.insert(self.dist.d(q, elem), vec![elem]);
                    }
                }
            }
            let mut refined = Vec::new();
            for bi in tree.values() {
                refined.push(bi.clone());
            }
            return refined;
        }

        fn separate_if_separable(&self, p: u32, x_prime: Vec<u32>) -> Option<Vec<VecPoint>> {
            if x_prime.len() == 0 {
                return Some(vec![]);
            }

            let x_min: u32 = *x_prime.first().unwrap();
            let x_max: u32 = *x_prime.last().unwrap();

            let b_delta = self.dist.d(x_min, x_max);
            let l_delta = self.dist.d(p, x_min);

            if b_delta <= l_delta {
                return Some(vec![VecPoint {
                    point: x_min,
                    vec: x_prime,
                }]);
            }
            for y_ind in 0..x_prime.len() - 1 {
                let y = x_prime[y_ind];
                let z = x_prime[y_ind + 1];

                if (self.dist.d(x_min, y) <= l_delta)
                    && (self.dist.d(z, x_max) <= l_delta)
                    && (self.dist.d(y, z) >= l_delta)
                {
                    let u1_vec = x_prime[..y_ind + 1].to_vec();
                    let u2_vec = x_prime[y_ind + 1..].to_vec();
                    return Some(vec![
                        VecPoint {
                            point: x_min,
                            vec: u1_vec,
                        },
                        VecPoint {
                            point: x_max,
                            vec: u2_vec,
                        },
                    ]);
                }
            }
            return None;
        }

        fn recursive_refine(
            &self,
            p: u32,
            in_list: Vec<u32>,
            s_list: Vec<u32>,
            out_list: Vec<u32>,
        ) -> Vec<Vec<u32>> {
            if in_list.len() == 0 && out_list.len() == 0 {
                return vec![s_list];
            } else {
                let q: u32;

                match in_list.first() {
                    Some(l) => q = *l,
                    None => q = *out_list.first().unwrap(),
                }

                let s_list_list = self.refine(q, s_list);

                let mut s_prime_list_list: Vec<Vec<u32>>;
                if out_list.contains(&q) {
                    let mut alpha = s_list_list.len();
                    for j in 0..s_list_list.len() {
                        if self.dist.d(*s_list_list[j].first().unwrap(), q) > self.dist.d(p, q) {
                            alpha = j;
                            break;
                        }
                    }

                    s_prime_list_list = Vec::new();
                    for i in 1..alpha + 1 {
                        s_prime_list_list.push(s_list_list[alpha - i].to_vec());
                    }
                    for i in alpha..s_list_list.len() {
                        s_prime_list_list.push(s_list_list[i].to_vec());
                    }
                } else {
                    s_prime_list_list = s_list_list;
                }

                let mut ti_list: Vec<Vec<Vec<u32>>> = Vec::new();

                let mut in_list_new = in_list.clone();
                in_list_new.retain(|&x| x != q);

                let mut out_list_new = out_list.clone();
                out_list_new.retain(|&x| x != q);
                for i in 0..s_prime_list_list.len() {
                    let ini = [s_prime_list_list[0..i].concat(), in_list_new.clone()].concat();
                    let outi = [s_prime_list_list[i + 1..].concat(), out_list_new.clone()].concat();
                    ti_list.push(self.recursive_refine(p, ini, s_prime_list_list[i].clone(), outi));
                }

                return ti_list.concat();
            }
        }

        fn sort_by_bipartition(&self, p: u32, x_list: Vec<u32>) -> Vec<u32> {
            //avec x_list qui est en fait X/{p}
            let mut l_list = Vec::new();
            let mut r_list = Vec::new();
            let mut undecided = x_list.clone();
            undecided.reverse();

            for q in &undecided.clone() {
                if (&undecided).contains(q) {
                    r_list.insert(0, *q);
                    undecided.retain(|&x| x != *q);
                }
                let mut skipped = Vec::new();
                let does_l_contain_q = l_list.contains(q);
                let does_r_contain_q = r_list.contains(q);

                for x in undecided {
                    if self.dist.d(x, *q) == self.dist.d(p, *q) {
                        skipped.insert(0, x);
                    } else {
                        if (self.dist.d(x, *q) < self.dist.d(p, *q) && does_l_contain_q)
                            || (self.dist.d(x, *q) > self.dist.d(p, *q) && does_r_contain_q)
                        {
                            l_list.insert(0, x);
                            r_list = [skipped.clone(), r_list].concat();
                        } else {
                            r_list.insert(0, x);
                            l_list = [skipped.clone(), l_list].concat();
                        }
                        skipped = Vec::new();
                    }
                }
                skipped.reverse();
                undecided = skipped;
            }
            l_list.reverse();
            return [l_list, [p].to_vec(), r_list].concat();
        }

        fn find_compatible_order(&self, x_list: Vec<u32>) -> Vec<u32> {
            if x_list.len() == 0 {
                return vec![];
            } else {
                let p = *x_list.first().unwrap();
                let x_prime_list = x_list[1..].to_vec();
                let mut c_list = self.recursive_refine(p, vec![p], x_prime_list, vec![]);
                let mut represented_copoints: Vec<VecPoint> = Vec::new();

                for i in (0..c_list.len()).rev() {
                    let c_prime_i =
                        self.find_compatible_order(std::mem::replace(&mut c_list[i], vec![]));
                    represented_copoints = [
                        match self.separate_if_separable(p, c_prime_i) {
                            Some(l) => l,
                            None => vec![],
                        },
                        represented_copoints,
                    ]
                    .concat()
                }

                let mut points_list: Vec<u32> = Vec::new();
                let mut vec_list: Vec<Vec<u32>> = Vec::new();

                for copoint in represented_copoints {
                    points_list.push(copoint.point);
                    vec_list.push(copoint.vec);
                }

                let points_sorted_list = self.sort_by_bipartition(p, points_list.clone());

                let mut compatible_order: Vec<Vec<u32>> = Vec::new();

                for point_sorted in points_sorted_list {
                    if point_sorted == p {
                        compatible_order.push(vec![p]);
                        continue;
                    }
                    for point_ind in 0..points_list.len() {
                        if points_list[point_ind] == point_sorted {
                            compatible_order
                                .push(std::mem::replace(&mut vec_list[point_ind], vec![]))
                        }
                    }
                }
                return compatible_order.concat();
            }
        }

        /// Tries to find a compatible order for the distance matrice.
        ///
        /// Return true if the matrice admit a compatible order, false otherwise.
        ///
        /// Stores the permutation (a compatible order if the distance matrice admit at least one) found in self.solved_permut
        ///
        /// If you want to resolve matrice bigger then a 1000x1000, you might need to increase your thread stack size. This is due to the depth of recursives function of this algorithm.
        pub fn resolve_robin(&mut self) -> bool {
            let mut x_list = Vec::new();
            let n = self.dist.distance_mat.len();
            for i in 1..n + 1 {
                x_list.push(i as u32);
            }
            let permut_found = self.find_compatible_order(x_list);

            let result_mat = self.dist.permut_matrice(&permut_found);

            self.solved_permut = Some(permut_found);

            return result_mat.is_robinson();
        }
    }
    ///This structure is just a `Vec<Vec<u32>>`.
    ///
    /// In the case of this crate, it'll always be a square matrice.
    pub struct DistanceMatrice {
        pub distance_mat: Vec<Vec<u32>>,
    }

    impl DistanceMatrice {
        fn d(&self, q: u32, x: u32) -> u32 {
            if q <= x {
                return self.distance_mat[(q - 1) as usize][(x - 1) as usize];
            } else {
                return self.distance_mat[(x - 1) as usize][(q - 1) as usize];
            }
        }

        pub fn permut_matrice(&self, permut: &Vec<u32>) -> DistanceMatrice {
            let mut new_matrice = get_empty_distance_matrice(self.distance_mat.len() as u32);

            for i in 0..new_matrice.len() {
                for j in i..new_matrice.len() {
                    new_matrice[i][j] = self.d(permut[i], permut[j]);
                }
            }

            return DistanceMatrice {
                distance_mat: new_matrice,
            };
        }

        ///Return true if the given `DistanceMatrice` respect the Robinson property, false otherwise
        pub fn is_robinson(&self) -> bool {
            for i in 1..self.distance_mat.len() {
                for j in i + 1..self.distance_mat.len() + 1 {
                    if self.d(i as u32, j as u32)
                        < max(
                            self.d((i + 1) as u32, j as u32),
                            self.d(i as u32, (j - 1) as u32),
                        )
                    {
                        return false;
                    }
                }
            }
            return true;
        }

        ///Display a `DistanceMatrice`
        pub fn display_mat(&self) {
            let mut complete_str = String::new();

            let mut max_len = 0;

            for line in &self.distance_mat {
                for elem in line {
                    if elem.to_string().len() > max_len {
                        max_len = elem.to_string().len();
                    }
                }
            }

            let width = max_len + 1;

            for line in &self.distance_mat {
                let mut line_str = String::new();
                for col in line {
                    line_str = format!(
                        "{}{}",
                        line_str,
                        format!("{:width$}", col.to_string(), width = width)
                    );
                }
                complete_str = format!("{}\n{}", complete_str, line_str);
            }
            println!("{}", complete_str);
        }
    }

    #[test]
    fn test_is_robinson_true() {
        let dist_mat = DistanceMatrice {
            distance_mat: vec![
                vec![0, 1, 2, 4],
                vec![0, 0, 2, 3],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
        };
        assert!(dist_mat.is_robinson());
    }

    #[test]
    fn test_refine() {
        let mat = vec![
            vec![0, 1, 2, 1],
            vec![0, 0, 2, 3],
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
        ];

        let rob = Robin::new(mat);

        assert_eq!(rob.refine(1, vec![2, 3, 4]), vec![vec![4, 2], vec![3]]);
    }

    #[test]
    fn test_recursive_refine() {
        let mat = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 2, 3],
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
        ];

        let rob = Robin::new(mat);

        assert_eq!(
            rob.recursive_refine(1, vec![1], vec![4, 3, 2], vec![]),
            vec![vec![2, 3, 4]]
        );
    }

    #[test]
    fn test_d() {
        let dist_mat = DistanceMatrice {
            distance_mat: vec![
                vec![0, 1, 2, 4],
                vec![0, 0, 3, 3],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
        };

        assert_eq!(dist_mat.d(1, 3), 2);
        assert_eq!(dist_mat.d(3, 1), 2);
    }

    #[test]
    fn test_permut_matrice() {
        let dist_mat = DistanceMatrice {
            distance_mat: vec![
                vec![0, 2, 2, 3, 4],
                vec![0, 0, 1, 1, 3],
                vec![0, 0, 0, 1, 2],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0],
            ],
        };

        assert_eq!(
            dist_mat.permut_matrice(&vec![4, 5, 3, 2, 1]).distance_mat,
            vec![
                vec![0, 1, 1, 1, 3],
                vec![0, 0, 2, 3, 4],
                vec![0, 0, 0, 1, 2],
                vec![0, 0, 0, 0, 2],
                vec![0, 0, 0, 0, 0]
            ]
        )
    }

    #[test]
    fn test_is_robinson_false() {
        let dist_mat = DistanceMatrice {
            distance_mat: vec![
                vec![0, 1, 2, 4],
                vec![0, 0, 3, 3],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
        };
        assert!(!dist_mat.is_robinson());
    }

    #[derive(Clone)]
    struct VecPoint {
        point: u32,
        vec: Vec<u32>,
    }
}
