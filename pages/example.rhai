fn update() {
    p(ui, "Hello");
    p(ui, "World");
    if button(ui, "test").clicked() {
        print("Button clicked");
    }
}