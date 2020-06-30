# Rust Domain Driven Design Example

The purpose of this project is to write a simple application in Rust (2018 edition) that follow basic Domain Driven Design principles, in order to test the capabilities of the language to write applications that follow DDD.


## Running the project

Clone the repository, and run in the root of the project:

```bash
cargo run
```

A menu will appear that will guide you through the application.

## Structure

The source code is explicitly split into three of the typical DDD layers:
- **Domain** - Where the business rules of the application reside.
- **Application** - The layer that orchestrates Domain and Infrastructure, and contains the use cases for your application.
- **Infrastructure** - Contains implementations of the abstractions defined in the Domain layer, and other infrastructure details. 

`main.rs` contains the initializations of the infrastructure implementations, and contains the UI implementation.


## Contribution
Please, feel free to contribute asking, discussing, commenting or improving the application via Pull Requests or Issues. All type of feedback will be welcomed!

## Resources

- [What is DDD - Talk by Eric Evans](https://www.youtube.com/watch?v=pMuiVlnGqjk)
- [Domain-Driven Design Reference card - DZone](https://dzone.com/refcardz/getting-started-domain-driven?chapter=7)
- [(a.k.a. The Blue Book) Domain-Driven Design: Tackling Complexity in the Heart of Software - Eric Evans](https://www.amazon.com/Domain-Driven-Design-Tackling-Complexity-Software/dp/0321125215)
- [(a.k.a. The Red Book) Implementing Domain-Driven Design - Vaughn Vernon](https://www.amazon.com/Implementing-Domain-Driven-Design-Vaughn-Vernon/dp/0321834577)