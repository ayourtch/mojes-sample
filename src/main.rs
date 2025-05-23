use iron::prelude::*;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use time::precise_time_ns;

use mojes::{js_type, to_js};

use mojes::dom::*;

// Simple function with basic operations
#[to_js]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// More complex function with control flow
#[to_js]
fn factorial(n: i32) -> i32 {
    let mut result = 1;
    let mut i = 1;

    while i <= n {
        result *= i;
        i += 1;
    }

    result
}

// Function using camelCase DOM API methods
#[to_js]
fn testFunc() {
    let element = document.getElementById("test");
    match element {
        Some(el) => {
            console.log(&format!("Found element with id: {}", el.id));
            alert(&format!("Test: {} - Element found!", factorial(6)));
        }
        None => {
            console.error("Element not found!");
            alert("Element not found!");
        }
    }
}

// DOM manipulation example with camelCase methods
#[to_js]
fn domExample() {
    let newElement = document.createElement("div");
    let elements = document.getElementsByTagName("p");

    for (i, element) in elements.iter().enumerate() {
        console.log(&format!(
            "Element {}: {}, {}",
            i, element.tagName, element.innerHTML
        ));
    }

    for e in elements {
        println!("New element: {}: {}", e.tagName, e.innerHTML);
    }

    // Query selector example
    let button = document.querySelector("#myButton");
    match button {
        Some(btn) => {
            console.log(&format!("Button found: {}", btn.id));
            btn.addEventListener("click", || {
                console.log("Button clicked!");
            });
        }
        None => {
            console.warn("Button not found");
        }
    }
}

// CSS styling example
#[to_js]
fn styleExample() {
    let element = document.getElementById("styledElement");
    match element {
        Some(el) => {
            let styles = window.getComputedStyle(&el);
            console.log(&format!("Current color: {}", styles.color));

            // Mock style manipulation (in real implementation, element.style would be mutable)
            console.log("Would set element.style.backgroundColor = 'red'");
            console.log("Would set element.style.fontSize = '20px'");
            //el.style.backgroundColor = "red";
        }
        None => {
            console.error("Styled element not found");
        }
    }
}

// Event handling example
#[to_js]
fn eventExample() {
    console.log("Event example");
    let elements = document.querySelectorAll(".clickable");
    for (index, element) in elements.iter().enumerate() {
        console.log(&format!("Adding event listener to element {}", index));
        element.addEventListener("click", || {
            console.log("Element clicked!");
        });
    }

    // Window events
    window.addEventListener("resize", || {
        console.log(&format!(
            "Window resized to: {}x{}",
            window.innerWidth(),
            window.innerHeight()
        ));
    });
}

// Timer functions example
#[to_js]
fn timerExample() {
    console.log("Setting up timers...");

    let timeoutId = setTimeout(
        || {
            console.log("Timeout fired!");
        },
        1000,
    );

    let intervalId = setInterval(
        || {
            console.log("Interval fired!");
        },
        500,
    );
    // Clear timers after some time (in a real app)
    setTimeout(
        move || {
            clearTimeout(timeoutId);
            clearInterval(intervalId);
            console.log("Timers cleared");
        },
        5000,
    );
}

// Navigation example
#[to_js]
fn navigationExample() {
    console.log(&format!("Current URL: {}", location.href));
    console.log(&format!("User Agent: {}", navigator.userAgent));
    console.log(&format!("Language: {}", navigator.language));

    if confirm("Do you want to reload the page?") {
        location.reload();
    }
}

// Form handling example
#[to_js]
fn formExample() {
    console.log("FORM");
    let form = document.querySelector("form");
    match form {
        Some(f) => {
            let inputs = f.querySelectorAll("input");
            for (i, input) in inputs.iter().enumerate() {
                console.log(&format!("Input {}: value = '{}'", i, input.value));
            }
        }
        None => {
            console.log("No form found");
        }
    }
}

// Animation example
#[to_js]
fn animationExample() {
    let element = document.getElementById("animatedElement");
    match element {
        Some(el) => {
            let mut position = 0;

            let mut animate = move || {
                position += 1;
                console.log(&format!("Animation frame: position = {}", position));

                if position < 100 {
                    requestAnimationFrame(|| {
                        // Recursive animation call would go here
                        console.log("Next animation frame requested");
                    });
                } else {
                    console.log("Animation complete");
                }
            };

            requestAnimationFrame(move || {
                animate();
            });
        }
        None => {
            console.error("Animated element not found");
        }
    }
}

// Local storage example (mock)
#[to_js]
fn storageExample() {
    // Note: localStorage would need to be added to the DOM API if needed
    console.log("Storage operations go here");
    // FIXME: add the test with if let in statement context
    let _x = if let Some(x) = localStorage.getItem("key") {
        println!("Local storage value: {}", x);
    } else {
        println!("Local storage value unset");
    };
    println!("X");
    localStorage.setItem("key", "value");
    println!("X1");
    // FIXME: Add tracking or something for duplicate variable declaration, or figure a workaround
    let _x1 = if let Some(x) = localStorage.getItem("key") {
        println!("Local storage value: {}", x);
    } else {
        println!("Local storage value unset");
    };
}

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    use iron::StatusCode;
    use iron::mime;
    let content_type = "text/html".parse::<mime::Mime>().unwrap();

    // Generate JavaScript with proper DOM API shims (no need for shimming since we use exact JS names)
    let js_code = format!(
        r#"
// Transpiled Rust functions with native JavaScript DOM API calls
{}
"#,
        JS.join("\n")
    );

    let data = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust-to-JS Transpiler Demo with CamelCase DOM API</title>
    <style>
        body {{ 
            font-family: Arial, sans-serif; 
            margin: 20px; 
            background-color: #f0f0f0;
        }}
        .container {{ 
            max-width: 800px; 
            margin: 0 auto; 
            background: white; 
            padding: 20px; 
            border-radius: 8px; 
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        button {{ 
            background: #007bff; 
            color: white; 
            border: none; 
            padding: 10px 20px; 
            margin: 5px; 
            border-radius: 4px; 
            cursor: pointer; 
            font-size: 14px;
        }}
        button:hover {{ 
            background: #0056b3; 
        }}
        .clickable {{
            background: #28a745;
            color: white;
            padding: 8px 16px;
            margin: 2px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }}
        .clickable:hover {{
            background: #1e7e34;
        }}
        #styledElement {{
            width: 200px;
            height: 100px;
            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
            margin: 10px 0;
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
        }}
        #animatedElement {{
            width: 50px;
            height: 50px;
            background: #ff6b6b;
            border-radius: 50%;
            margin: 10px 0;
            transition: transform 0.3s ease;
        }}
        #animatedElement:hover {{
            transform: scale(1.2);
        }}
        .demo-section {{
            margin: 20px 0;
            padding: 15px;
            border: 1px solid #ddd;
            border-radius: 6px;
            background: #f9f9f9;
        }}
        .demo-section h3 {{
            margin-top: 0;
            color: #333;
        }}
        input {{
            padding: 8px;
            margin: 5px;
            border: 1px solid #ddd;
            border-radius: 4px;
            width: 200px;
        }}
        form {{
            background: #f8f9fa;
            padding: 15px;
            border-radius: 6px;
            margin: 10px 0;
        }}
    </style>
    <script>{}</script>
</head>
<body>
    <div class="container">
        <h1>Rust-to-JavaScript Transpiler Demo</h1>
        <p>This demo shows Rust functions transpiled to JavaScript using <strong>native camelCase DOM API method names</strong>.</p>
        
        <div class="demo-section">
            <h3>Basic Functions</h3>
            <div id='test'>Test Element</div>
            <button onclick="testFunc()">Test Basic Function</button>
            <button onclick="console.log('Simple calculation: ' + add(5, 3))">Test Add Function</button>
            <button onclick="console.log('Factorial of 5: ' + factorial(5))">Test Factorial</button>
        </div>

        <div class="demo-section">
            <h3>DOM Manipulation</h3>
            <p>Paragraph 1</p>
            <p>Paragraph 2</p>
            <p>Paragraph 3</p>
            <button onclick="domExample()">Test DOM Operations</button>
            <button id="myButton" onclick="console.log('Button clicked directly!')">Target Button</button>
        </div>

        <div class="demo-section">
            <h3>CSS Styling</h3>
            <div id="styledElement">Styled Element</div>
            <button onclick="styleExample()">Test Style Operations</button>
        </div>

        <div class="demo-section">
            <h3>Event Handling</h3>
            <button class="clickable">Clickable 1</button>
            <button class="clickable">Clickable 2</button>
            <button class="clickable">Clickable 3</button>
            <br>
            <button onclick="eventExample()">Setup Event Listeners</button>
        </div>

        <div class="demo-section">
            <h3>Timers & Animation</h3>
            <div id="animatedElement"></div>
            <button onclick="timerExample()">Test Timers</button>
            <button onclick="animationExample()">Test Animation</button>
        </div>

        <div class="demo-section">
            <h3>Navigation & Browser Info</h3>
            <button onclick="navigationExample()">Test Navigation</button>
            <button onclick="console.log('Current page info logged to console')">Log Page Info</button>
        </div>

        <div class="demo-section">
            <h3>Form Handling</h3>
            <form>
                <label>Name: <input type="text" name="name" value="John Doe"></label><br>
                <label>Email: <input type="email" name="email" value="john@example.com"></label><br>
                <label>Age: <input type="number" name="age" value="30"></label><br>
            </form>
            <button onclick="formExample()">Read Form Values</button>
        </div>

        <div class="demo-section">
            <h3>Storage Operations</h3>
            <button onclick="storageExample()">Test Storage</button>
        </div>

        <div class="demo-section">
            <h3>Console Output</h3>
            <p>Open the browser's developer tools console (F12) to see the output from the Rust functions.</p>
            <button onclick="console.log('Manual console test from HTML')">Manual Console Test</button>
        </div>
    </div>

    <script>
        // Additional JavaScript to enhance the demo
        console.log('=== Rust-to-JS Transpiler Demo Started ===');
        
        // Add some interactivity to demonstrate the transpiled functions work correctly
        document.addEventListener('DOMContentLoaded', function() {{
            console.log('DOM loaded, all Rust-transpiled functions ready!');
            
            // Test that our transpiled functions exist
            if (typeof testFunc === 'function') {{
                console.log('✓ testFunc is available');
            }}
            if (typeof domExample === 'function') {{
                console.log('✓ domExample is available');
            }}
            if (typeof add === 'function') {{
                console.log('✓ add function is available, 2+3=' + add(2,3));
            }}
            if (typeof factorial === 'function') {{
                console.log('✓ factorial function is available, factorial(4)=' + factorial(4));
            }}
        }});

        // Window resize handler to demonstrate browser API integration
        window.addEventListener('resize', function() {{
            console.log('Window resized to: ' + window.innerWidth + 'x' + window.innerHeight);
        }});
    </script>
</body>
</html>"#,
        js_code
    );

    Ok(Response::with((content_type, StatusCode::OK, data)))
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    println!("🚀 Rust-to-JS Transpiler Server starting...");
    println!("📊 Server running on http://localhost:3000");
    println!("🔧 DOM API uses native JavaScript camelCase method names");
    println!("🎯 Open browser developer tools to see console output");
    Iron::new(chain).http("localhost:3000");
}
