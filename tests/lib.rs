use yate::html;

#[test]
fn test() {
    let world = "planet";
    assert_eq!(
        html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <title>"Example"</title>
                </head>
                <body>
                    <!-- "comment" -->
                    <div hello={world} />
                    <>
                        <div>"1"</div>
                        <div>"2"</div>
                        <div>"3"</div>
                    </>
                </body>
            </html>
        },
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Example</title>
                </head>
                <body>
                    <!-- comment -->
                    <div hello="planet"></div>
                    <div>1</div>
                    <div>2</div>
                    <div>3</div>
                </body>
            </html>
        "#
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("")
    );
}
