
Implementando Formas Geométricas com Traits em Rust
Entendendo o Problema
Queremos criar uma estrutura em Rust que represente formas geométricas de forma genérica, utilizando o conceito de traits. Cada forma geométrica deverá ter métodos para calcular sua área, perímetro e, para fins de visualização, um método para "desenhar" a forma.

Solução em Rust
Rust
use std::fmt;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn draw(&self) -> String;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width   
 + self.height)
    }

    fn draw(&self) -> String {
        format!("Rectangle: width = {}, height = {}", self.width, self.height)
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)   

    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn draw(&self) -> String {
        format!("Circle: radius = {}", self.radius)
    }
}

fn main() {
    let rect = Rectangle { width: 5.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    println!("Área do retângulo: {}", rect.area());
    println!("Perímetro do retângulo: {}", rect.perimeter());
    println!("{}", rect.draw());

    println!("Área do círculo: {}", circle.area());
    println!("Perímetro do círculo: {}", circle.perimeter());
    println!("{}", circle.draw());
}
Use o código com cuidado.

Explicação:
Trait Shape: Define os métodos que todas as formas geométricas devem implementar: area, perimeter e draw.
Estruturas Rectangle e Circle: Representam as formas geométricas específicas.
Implementação do trait: Cada estrutura implementa o trait Shape, fornecendo as implementações específicas para seus métodos.
Método draw: Neste exemplo, draw simplesmente retorna uma string descrevendo a forma. Você poderia implementar uma lógica mais complexa para desenhar a forma em um terminal ou em uma interface gráfica.
Função main: Cria instâncias de Rectangle e Circle e demonstra o uso dos métodos.
Benefícios do uso de traits:
Polimorfismo: Permite tratar diferentes formas de maneira uniforme, através do trait Shape.
Reusabilidade: O código é mais reutilizável, pois podemos criar novas formas implementando o trait Shape.
Extensibilidade: É fácil adicionar novas formas ao sistema.
Abstração: Esconde a complexidade de cada forma, expondo apenas a interface comum definida pelo trait.
Próximos passos:
Mais formas: Adicione outras formas geométricas, como triângulos, quadrados, etc.
Hierarquia de classes: Crie uma hierarquia de classes para representar relações entre formas (por exemplo, um quadrado é um tipo especial de retângulo).
Visualização: Implemente o método draw para gerar representações visuais das formas, usando uma biblioteca gráfica como o raylib.
Operações geométricas: Adicione métodos para realizar operações geométricas, como rotação, translação, etc.
Com este exemplo, você tem uma base sólida para construir um sistema mais complexo de geometria em Rust, utilizando a poderosa ferramenta dos traits.
