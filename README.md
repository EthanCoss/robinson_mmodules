
# Robinson

This rust crate is a direct implementation of the algorithms presented in “MODULES IN ROBINSON SPACES” by MIKHAEL CARMONA, VICTOR CHEPOI, GUYSLAIN NAVES AND PASCAL PREA.

Given a dissimilarity space (~ what I call a DistanceMatrice), this algorithm can recognize if this dissimilarity space is Robinson.


## Authors

- [@EthanCoss](https://github.com/EthanCoss)
- Special thanks to [@FrancoisBrucker](https://github.com/FrancoisBrucker) and Pascal Préa who help me and provided me necesssary ressources.


## Installation/Dependencies

You can add robinson_mmodules to your dependencies using
```bash
cargo add robinson_mmodules
```
in your project directory.

Or add Robinson to your dependencies directly in your cargo.toml file
```toml
[dependencies]
robinson_mmodules = "0.X.0"
```

## Usage/Examples

```rust
extern crate robinson_mmodules;
use robinson_mmodules::Robin;

fn main() {
    let my_matrice = vec![
        vec![0, 1, 2, 4, 3],
        vec![0, 0, 1, 3, 1],
        vec![0, 0, 0, 2, 1],
        vec![0, 0, 0, 0, 2],
        vec![0, 0, 0, 0, 0],
    ];
    let mut robin = Robin::new(my_matrice);

    println!("{}", robin.resolve_robin()); //true if your matrice admit a compatible order (equivalent to if your dissimilarity space is Robinson), false otherwise. In this case, it's true.
    println!("{:?}", robin.solved_permut.clone().unwrap()); //[1, 2, 3, 5, 4] which is a valid permutation
    //That means that if we apply this permutation we will get a matrice that respect the Robinson property
    let robinson_distance_matrice = robin.dist.permut_matrice(&robin.solved_permut.unwrap());
    println!("{}", robinson_distance_matrice.is_robinson()); //True since this DistanceMatrice respect the Robinson property here
    robinson_distance_matrice.display_mat(); //As you can visualize here
}

```

You can also find a more in depth exemple in the main.rs and test.rs file of this github repository. If you want to try this one yourself you'll need to add the rand crate to your depencies `rand = "0.8.5"`

## Documentation

[Documentation](https://docs.rs/robinson_mmodules/latest/robinson_mmodules/)


## License

[MIT](https://choosealicense.com/licenses/mit/)