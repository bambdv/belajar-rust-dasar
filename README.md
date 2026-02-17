# Belajar Rust Dasar 🦀

Dokumentasi lengkap tentang pembelajaran fundamental Rust programming language.

---

## 📋 Daftar Isi

1. [Modul & Import](#-modul--import)
2. [Variabel & Tipe Data](#-variabel--tipe-data)
3. [Operasi & Operator](#-operasi--operator)
4. [Kontrol Alur](#-kontrol-alur)
5. [Fungsi](#-fungsi)
6. [String & Slice](#-string--slice)
7. [Struct](#-struct)
8. [Enum & Pattern Matching](#-enum--pattern-matching)
9. [Trait](#-trait)
10. [Generics](#-generics)
11. [Lifetime](#-lifetime)
12. [Operator Overloading](#-operator-overloading)
13. [Optional & Result](#-optional--result)
14. [Koleksi Data](#-koleksi-data)
15. [Iterator](#-iterator)
16. [Error Handling](#-error-handling)
17. [Smart Pointers](#-smart-pointers)
18. [Concurrency & Static](#-concurrency--static)
19. [Unsafe & Macro](#-unsafe--macro)

---

## 🔧 Modul & Import

### Pengertian

Modul adalah cara untuk mengorganisir kode Rust ke dalam namespace yang berbeda, membantu mengelola kompleksitas dan meningkatkan reusability.

### Contoh Penggunaan

```rust
// Import satu fungsi
use first::say_hello;

// Rename saat import
use second::say_hello as say_hello_second;

// Import semua member
use model::*;

// Import spesifik
use model::{User, test};
```

### Struktur Modul

- `first.rs` - Module dengan sub-module nested
- `model.rs` - Module dengan struct dan implementasi
- `second.rs` - Module dengan fungsi publik
- `third.rs` - Module yang diimport oleh module lain

---

## 📝 Variabel & Tipe Data

### Variabel Immutable

Secara default, variabel di Rust bersifat immutable (tidak dapat diubah).

```rust
let x = 5; // tidak bisa diubah
```

### Variabel Mutable

Gunakan keyword `mut` untuk membuat variabel yang dapat diubah.

```rust
let mut x = 5;
x = 10; // legal
```

### Shadowing

Rust memungkinkan Anda untuk mendeklarasikan variabel baru dengan nama yang sama, yang akan "menyembunyikan" variabel sebelumnya.

```rust
let x = 5;
let x = x + 1; // x sekarang 6
let x = "string"; // x sekarang string
```

### Tipe Data Primitif

- **Integer**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
- **Float**: `f32`, `f64`
- **Boolean**: `true`, `false`
- **Character**: Nilai Unicode tunggal
- **String**: Text data
- **Tuple**: Koleksi fixed-size dengan tipe berbeda
- **Array**: Koleksi fixed-size dengan tipe sama

### Tipe Eksplisit

Rust memiliki type inference yang kuat, tapi Anda bisa menentukan tipe secara eksplisit.

```rust
let age: u8 = 25;
let height: f64 = 175.5;
```

### Konstanta

Konstanta adalah nilai yang tidak dapat diubah dan harus di-declare dengan tipe data.

```rust
const MINIMUM: i32 = 0;
```

### Unit Type

Unit adalah tuple kosong `()`, digunakan ketika fungsi tidak mengembalikan nilai.

---

## 🔢 Operasi & Operator

### Operator Aritmatika

```rust
let a = 10;
let b = 3;

let add = a + b;      // 13
let sub = a - b;      // 7
let mul = a * b;      // 30
let div = a / b;      // 3
let rem = a % b;      // 1
```

### Operator Perbandingan

```rust
let hasil = a == b;   // false
let hasil = a != b;   // true
let hasil = a < b;    // false
let hasil = a > b;    // true
let hasil = a <= b;   // false
let hasil = a >= b;   // true
```

### Operator Logika Boolean

```rust
let a = true;
let b = false;

let hasil = a && b;   // false (AND)
let hasil = a || b;   // true (OR)
let hasil = !a;       // false (NOT)
```

### Augmented Assignment

```rust
let mut x = 5;
x += 3;  // x = 8
x -= 2;  // x = 6
x *= 2;  // x = 12
x /= 3;  // x = 4
x %= 3;  // x = 1
```

### Konversi Tipe (Type Conversion)

```rust
let a: i32 = 42;
let b: u8 = a as u8;
let c: i64 = a as i64;
```

---

## 🔀 Kontrol Alur

### If Expression

```rust
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}
```

### If dalam Let Statement

```rust
let number = if condition { 5 } else { 6 };
```

### Loop Expression

Infinite loop yang dapat dikontrol dengan break dan continue.

```rust
loop {
    if condition {
        break;
    }
}

// Return value dari loop
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

### Loop dengan Label

Label memungkinkan Anda untuk mengatasi nested loop.

```rust
'outer: loop {
    'inner: loop {
        break 'outer; // break outer loop
    }
}
```

### While Loop

```rust
while condition {
    // code
}
```

### For Loop dengan Array

```rust
let array = [1, 2, 3, 4, 5];
for element in array {
    println!("{}", element);
}
```

### Range

```rust
for i in 0..5 {           // 0, 1, 2, 3, 4 (exclusive)
    println!("{}", i);
}

for i in 0..=5 {          // 0, 1, 2, 3, 4, 5 (inclusive)
    println!("{}", i);
}

for i in (0..5).rev() {   // 4, 3, 2, 1, 0 (reverse)
    println!("{}", i);
}
```

---

## 🎯 Fungsi

### Function Parameter & Return Type

```rust
fn function_b(name: &str, age: u8) -> &str {
    println!("Name: {}, Age: {}", name, age);
    name
}
```

### Function Statement vs Expression

```rust
fn function_a() {
    let x = 3 + 6;       // statement (semicolon)
    println!("{}", x);
}

fn function_b() -> i32 {
    3 + 6                // expression (no semicolon)
}
```

### Factorial dengan Loop

```rust
fn factorial_loop(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
```

### Factorial dengan Rekursi

```rust
fn factorial_recursive(n: i32) -> i32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial_recursive(n - 1),
    }
}
```

### Ownership dalam Fungsi

```rust
fn hi(name: String) {
    println!("Hello {}", name);
    // name di-drop di sini
}

fn print_text(value: String, times: u32) {
    for _ in 0..times {
        println!("{}", value);
    }
}
```

---

## 📚 String & Slice

### String Slice (&str)

String slice adalah reference ke bagian dari string.

```rust
let s = "Hello, World!";
let hello = &s[0..5];      // "Hello"
let world = &s[7..12];     // "World"
```

### String Type

`String` adalah owned type yang dapat di-mutate dan grow.

```rust
let mut s = String::new();
s.push_str("Hello");
s += " World";
```

### Reference ke String

```rust
fn print_text(s: &str) {
    println!("{}", s);
}
```

### Mutable Reference ke String

```rust
fn change_value_mutable(value: &mut String) {
    value.push_str(" - modified");
}
```

---

## 🏗️ Struct

### Struct Definition

```rust
struct Person {
    name: String,
    age: u8,
    email: String,
}
```

### Struct Implementation

```rust
impl Person {
    fn new(name: String, age: u8, email: String) -> Person {
        Person { name, age, email }
    }
}
```

### Method dalam Struct

```rust
impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
```

### Tuple Struct

Struct tanpa field names.

```rust
struct GeoPoint(f64, f64);

let point = GeoPoint(10.5, 20.3);
println!("{}, {}", point.0, point.1);
```

### Unit Struct

Struct tanpa fields.

```rust
struct Nothing;
```

---

## 🔤 Enum & Pattern Matching

### Enum Definition

```rust
enum Level {
    Info,
    Warning,
    Error,
    Critical,
}
```

### Enum dengan Data

```rust
enum Payment {
    CreditCard { card_number: String, cvv: String },
    BankTransfer { account: String, routing: String },
    Wallet { phone: String },
}
```

### Match Expression

Pattern matching untuk menangani berbagai varian enum.

```rust
match payment {
    Payment::CreditCard { card_number, cvv } => {
        println!("Pay with credit card");
    }
    Payment::BankTransfer { account, routing } => {
        println!("Bank transfer");
    }
    _ => println!("Other payment method"),
}
```

### Pattern Matching dengan Range

```rust
match age {
    0..=12 => println!("Child"),
    13..=19 => println!("Teenager"),
    20..=64 => println!("Adult"),
    _ => println!("Senior"),
}
```

### Struct Pattern Matching

```rust
let Person { name, age, email } = person;
```

### Ignoring Values dalam Pattern

```rust
match value {
    (0, _) => println!("First is zero"),
    (_, 0) => println!("Second is zero"),
    _ => println!("Other values"),
}
```

---

## 📦 Trait

### Trait Definition

Trait mendefinisikan set method yang harus diimplementasikan oleh tipe.

```rust
trait CanSayHello {
    fn say_hello(&self);
}
```

### Trait Implementation

```rust
impl CanSayHello for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
```

### Multiple Traits

```rust
fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    value.say_hello();
    value.say_goodbye();
}
```

### Trait Bounds

```rust
trait CanSay: CanSayHello + CanSayGoodBye {
    fn say_name(&self);
}
```

### Return Trait

```rust
fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson { name }
}
```

---

## 🎛️ Generics

### Generic Struct

```rust
struct Point<T = i32> {
    x: T,
    y: T,
}

let point = Point { x: 5, y: 10 };
```

### Generic Enum

```rust
enum Value<T> {
    Some(T),
    None,
}
```

### Generic Function

```rust
fn min<T>(value1: T, value2: T) -> T
where
    T: PartialOrd,
{
    if value1 < value2 {
        value1
    } else {
        value2
    }
}
```

### Generic dengan Trait Bound

```rust
struct Hi<T = SimplePerson>
where
    T: CanSayGoodBye,
{
    value: T,
}
```

### Generic Method

```rust
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}
```

### Generic Trait Implementation

```rust
trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> T {
        self.x
    }
}
```

---

## ⏱️ Lifetime

### Lifetime Annotation Dasar

```rust
fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}
```

### Dangling Reference

Reference tidak boleh menunjuk ke value yang sudah di-drop.

```rust
// TIDAK VALID:
fn invalid() -> &str {
    let s = String::from("hello");
    &s  // Error: dangling reference
}
```

### Struct dengan Lifetime

```rust
struct Student<'a, 'b> {
    name: &'a str,
    school: &'b str,
}
```

### Lifetime dengan Generic

```rust
struct Teacher<'a, ID>
where
    ID: Ord,
{
    name: &'a str,
    id: ID,
}
```

---

## ➕ Operator Overloading

### Add Operator

```rust
impl<'a, 'b> Add<&'b Apple> for &'a Apple {
    type Output = Apple;

    fn add(self, other: &'b Apple) -> Apple {
        Apple {
            weight: self.weight + other.weight,
        }
    }
}
```

### Sub Operator

```rust
impl<'a, 'b> Sub<&'b Apple> for &'a Apple {
    type Output = Apple;

    fn sub(self, other: &'b Apple) -> Apple {
        Apple {
            weight: self.weight - other.weight,
        }
    }
}
```

---

## 🎁 Optional & Result

### Option Type

Rust tidak memiliki null. Solusinya menggunakan `Option<T>`.

```rust
fn double(n: Option<i32>) -> Option<i32> {
    match n {
        Some(value) => Some(value * 2),
        None => None,
    }
}

let result = double(Some(5));  // Some(10)
let result = double(None);     // None
```

### Option Manipulation

```rust
let value = Some(5);
value.map(|x| x * 2);          // Some(10)
```

### Result Type

Digunakan untuk error handling.

```rust
fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(h) => Ok(format!("Connected to {}", h)),
        None => Err("Host not provided".to_string()),
    }
}
```

---

## 📊 Koleksi Data

### Vector

Dynamic array yang dapat grow.

```rust
let mut vec = vec![1, 2, 3];
vec.push(4);
vec.pop();
let element = vec[0];
```

### VecDeque

Double-ended queue.

```rust
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();
```

### LinkedList

Doubly-linked list.

```rust
let mut list = LinkedList::new();
list.push_back(1);
list.push_front(0);
list.pop_back();
```

### HashMap

Key-value store dengan hashing.

```rust
let mut map = HashMap::new();
map.insert("name", "John");
map.insert("age", "30");
let value = map.get("name");
```

### BTreeMap

Key-value store dengan tree structure (sorted).

```rust
let mut map = BTreeMap::new();
map.insert(1, "one");
map.insert(2, "two");
```

### HashSet

Unordered set tanpa duplicates.

```rust
let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.contains(&1);
```

### BTreeSet

Ordered set (sorted).

```rust
let mut set = BTreeSet::new();
set.insert(1);
set.insert(2);
```

---

## 🔄 Iterator

### Iterator Basics

```rust
let vec = vec![1, 2, 3, 4, 5];
for num in vec.iter() {
    println!("{}", num);
}
```

### Iterator Methods

```rust
vec.iter().map(|x| x * 2);           // transform elements
vec.iter().filter(|x| x > &2);       // filter elements
vec.iter().fold(0, |acc, x| acc + x); // reduce to single value
```

### Collecting Results

```rust
let vec: Vec<i32> = (0..5).map(|x| x * 2).collect();
```

---

## ⚠️ Error Handling

### Recoverable Errors

```rust
fn connect_application(host: Option<String>) -> Result<String, String> {
    let cache = connect_cache(host.clone())?;
    let email = connect_email(host.clone())?;
    Ok(format!("Connected to cache and email"))
}
```

### Error Propagation dengan `?` Operator

`?` operator akan return error jika Result adalah Err.

---

## 🎯 Smart Pointers

### Box

Allocate value pada heap.

```rust
let b = Box::new(5);
println!("{}", b);
```

### Deref Trait

Memungkinkan dereferencing otomatis.

```rust
impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
```

### Drop Trait

Cleanup ketika value di-drop.

```rust
impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping: {}", self.title);
    }
}
```

### RC (Reference Count)

Multiple ownership.

```rust
let rc1 = Rc::new(5);
let rc2 = rc1.clone();
```

### RefCell

Interior mutability pattern.

```rust
let seller = RefCell::new(Seller {
    name: "John".to_string(),
    qty: 5,
});

seller.try_borrow_mut().unwrap().qty = 10;
```

---

## 🔗 Concurrency & Static

### Static Variable

```rust
static APPLICATION: &str = "my App";
```

### Mutex untuk Thread Safety

```rust
static COUNTER: Mutex<u32> = Mutex::new(0);

fn increment() {
    let mut num = COUNTER.lock().unwrap();
    *num += 1;
}
```

---

## 🚫 Unsafe & Macro

### Unsafe Code

```rust
let mut num = 5;
unsafe {
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32 as *const i32;
    println!("{}", *r1);
}
```

### Macro

Define custom syntax.

```rust
macro_rules! hi {
    () => {
        println!("Hello, Macro!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

hi!();
hi!("Rust");
```

---

## 📌 Traits & Derive

### Derive Attributes

Otomatis implement common traits.

```rust
#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    id: u32,
    name: String,
}
```

### Custom Debug Implementation

```rust
impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category({})", self.name)
    }
}
```

---

## 🎓 Closure

### Basic Closure

```rust
let add = |x: i32, y: i32| -> i32 {
    x + y
};

let result = add(2, 3);
```

### Closure dengan Captured Variables

```rust
let x = 5;
let closure = |y| y + x;
```

### Function Pointer sebagai Parameter

```rust
fn print_with_format(value: String, format: fn(String) -> String) {
    println!("{}", format(value));
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

print_with_format("hello".to_string(), to_uppercase);
```

### Closure sebagai Parameter

```rust
fn apply<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(value)
}
```

---

## 💾 Type Alias

### Simple Type Alias

```rust
type Age = u8;
type IdentityNumber = String;

let my_age: Age = 25;
```

### Struct Type Alias

```rust
struct Customer {
    name: String,
}

type Pelanggan = Customer;
```

---

## 🔍 Comparison Traits

### PartialEq

```rust
impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
```

### PartialOrd

```rust
impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
```

---

## 🎨 Format & Display

### String Formatting

```rust
let name = "Rust";
let version = 1;
println!("Language: {}, Version: {}", name, version);
println!("Debug: {:?}", some_struct);
```

---

## 🚀 Kesimpulan

Anda telah mempelajari konsep-konsep fundamental Rust:

- ✅ Memory safety tanpa garbage collector
- ✅ Ownership dan borrowing system
- ✅ Type system yang powerful
- ✅ Functional programming paradigms
- ✅ Error handling dengan Option dan Result
- ✅ Advanced features seperti lifetime, trait, generics
- ✅ Collections dan iterators
- ✅ Smart pointers dan concurrency

Selamat melanjutkan perjalanan belajar Rust! 🚀

---

**Dibuat pada**: 17 Februari 2026  
**Language**: Rust Programming Language
