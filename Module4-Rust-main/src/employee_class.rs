
    // Define the Employee struct
    #[derive(Debug)]
    struct Employee {
        name: String,
        salary: f64,
        id: u32,
        employee_type: EmployeeType,
    }

    // Define the EmployeeType enum
    #[derive(Debug)]
    enum EmployeeType {
        JuniorEngineer,
        SeniorEngineer,
    }

    // Implement a method to add salaries to the employee type
    impl Employee {
        fn add_salaries(&mut self, amount: f64) {
            self.salary += amount;
        }
        fn sub_salaries(&mut self, amount: f64) {
            self.salary -= amount;
        }
    }

pub fn employee_struct_class(){   
    // Create an instance of the Employee struct
    let mut employee1 = Employee {
        name: String::from("John Doe"),
        salary: 50000.0,
        id: 1,
        employee_type: EmployeeType::JuniorEngineer,
    };

    let mut employee2 = Employee {
        name: String::from("Alan Wake"),
        salary: 100000.0,
        id: 2,
        employee_type: EmployeeType::SeniorEngineer,
    };

    // Add salary to the employee 1 salary
    employee1.add_salaries(10000.0);

    // Subtract salary from the employee 2
    employee2.sub_salaries(10000.0);

    // Print the updated employee information
    println!("Employee 1:");
    println!("Name: {}", employee1.name);
    println!("Salary: ${:.2}", employee1.salary);
    println!("ID: {}", employee1.id);
    println!("Employee Type: {:?}", employee1.employee_type);
    println!("=============================================");    
    println!("Employee 2:");
    println!("Name: {}", employee2.name);
    println!("Salary: ${:.2}", employee2.salary);
    println!("ID: {}", employee2.id);
    println!("Employee Type: {:?}", employee2.employee_type);
}