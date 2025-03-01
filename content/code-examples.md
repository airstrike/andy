---
title: Code Examples with Syntax Highlighting
date: 2024-03-01T14:00:00Z
description: A showcase of syntax highlighting for various programming languages
slug: code-examples
---

# Code Examples with Syntax Highlighting

This post demonstrates the syntax highlighting feature of the blog for various programming languages.

## Rust

```rust
// A simple Rust function
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Using a struct
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
    
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }
}

fn main() {
    let person = Person::new("Alice", 30);
    println!("{}", person.greet());
    
    // Calculate and print the 10th Fibonacci number
    println!("The 10th Fibonacci number is: {}", fibonacci(10));
}
```

## JavaScript

```javascript
// JavaScript example
function createCounter() {
    let count = 0;
    
    return {
        increment() {
            count += 1;
        },
        decrement() {
            count -= 1;
        },
        getCount() {
            return count;
        }
    };
}

// Using the counter
const counter = createCounter();
counter.increment();
counter.increment();
console.log(counter.getCount()); // 2

// Modern JavaScript with async/await
async function fetchData(url) {
    try {
        const response = await fetch(url);
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching data:', error);
        return null;
    }
}

// Using the fetch function
fetchData('https://api.example.com/data')
    .then(data => {
        if (data) {
            console.log('Data:', data);
        }
    });
```

## Python

```python
# Python example
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)

# Using a class
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, my name is {self.name} and I am {self.age} years old."
    
    @classmethod
    def from_birth_year(cls, name, birth_year):
        import datetime
        current_year = datetime.datetime.now().year
        age = current_year - birth_year
        return cls(name, age)

# Testing the code
if __name__ == "__main__":
    # Sort a list
    numbers = [3, 6, 8, 10, 1, 2, 1]
    print(quicksort(numbers))
    
    # Create a person
    person = Person("Bob", 25)
    print(person.greet())
```

## HTML

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Example Page</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <h1>Welcome to my page</h1>
        <nav>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    </header>
    
    <main>
        <section class="hero">
            <h2>Hello, world!</h2>
            <p>This is a simple HTML example for syntax highlighting.</p>
            <button id="btnClick">Click me</button>
        </section>
    </main>
    
    <footer>
        <p>&copy; 2024 Example Inc.</p>
    </footer>
    
    <script src="script.js"></script>
</body>
</html>
```

## CSS

```css
/* CSS example */
:root {
    --primary-color: #3498db;
    --secondary-color: #2c3e50;
    --text-color: #333;
    --background-color: #fff;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Arial', sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--background-color);
}

header {
    background-color: var(--secondary-color);
    color: white;
    padding: 1rem 0;
}

nav ul {
    display: flex;
    list-style: none;
    justify-content: center;
}

nav li {
    margin: 0 1rem;
}

nav a {
    color: white;
    text-decoration: none;
    transition: color 0.3s ease;
}

nav a:hover {
    color: var(--primary-color);
}

.hero {
    text-align: center;
    padding: 2rem;
}

@media (max-width: 768px) {
    nav ul {
        flex-direction: column;
        align-items: center;
    }
    
    nav li {
        margin: 0.5rem 0;
    }
}
```

That concludes our syntax highlighting examples for different programming languages!