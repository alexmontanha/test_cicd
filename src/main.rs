fn main() {
    println!("Sistema de cálculo de Horas de PJ");
    println!("Bem-vindo ao sistema de cálculo de horas de PJ");
    println!("Digite o número de horas trabalhadas: ");
    
    let mut horas = String::new();
    std::io::stdin().read_line(&mut horas).unwrap();
    let horas: f64 = horas.trim().parse().unwrap();

    println!("Digite o valor da hora: ");
    let mut valor = String::new();
    std::io::stdin().read_line(&mut valor).unwrap();
    let valor: f64 = valor.trim().parse().unwrap();
    let aliquota = 0.275;
    
    let salario = calcula_salario_bruto(horas, valor);

    let irrf = calcula_irrf(salario, aliquota);

    let salario_liquido = calcula_salario_liquido(salario, irrf);

    println!("O salário bruto é: R$ {} muito né?", salario);
    println!("O IRRF é: R$ {}", irrf);
    println!("O salário líquido é: R$ {}", salario_liquido);

}

fn calcula_salario_liquido(salario: f64, irrf: f64) -> f64 {
    let salario_liquido = salario - irrf;
    salario_liquido
}

fn calcula_irrf(salario: f64, aliquota: f64) -> f64 {
    let irrf = salario * aliquota;
    irrf
}

fn calcula_salario_bruto(horas: f64, valor: f64) -> f64 {
    let salario = horas * valor;
    salario
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcula_salario_bruto() {
        assert_eq!(calcula_salario_bruto(10.0, 100.0), 1000.0);
    }

    #[test]
    fn test_calcula_irrf() {
        assert_eq!(calcula_irrf(1000.0, 0.275), 275.0);
    }

    #[test]
    fn test_calcula_salario_liquido() {
        assert_eq!(calcula_salario_liquido(1000.0, 275.0), 725.0);
    }
}