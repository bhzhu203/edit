#[cfg(test)]
mod improved_trigger_tests {
    use edit::buffer::TextBuffer;
    use edit::helpers::Point;

    #[test]
    fn test_improved_auto_trigger() {
        println!("=== Testing Improved Auto-Trigger ===");
        
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        
        // Add test content
        buffer.write_canon(b"function test_function() {\n    let test_value = 42;\n    return test_;\n}");
        
        println!("Buffer content:");
        println!("{}", String::from_utf8_lossy(buffer.read_forward(0)));
        
        // Test 1: Single character trigger
        buffer.cursor_move_to_logical(Point { x: 11, y: 2 }); // After "test_"
        println!("\nTest 1: Typing single character 'f'");
        buffer.write_canon(b"f"); // This should trigger auto-completion
        
        println!("Is completing after 'f': {}", buffer.is_completing());
        if buffer.is_completing() {
            let state = buffer.get_auto_completion_state();
            println!("  Prefix: '{}'", state.prefix);
            println!("  Suggestions: {}", state.items.len());
            for (i, item) in state.items.iter().enumerate() {
                println!("    {}: {}", i, item.label);
            }
        }
        
        // Test 2: Reset and try with Ctrl+Space
        buffer.cancel_auto_completion();
        println!("\nTest 2: Manual trigger with Ctrl+Space");
        buffer.trigger_auto_completion();
        println!("Is completing with manual trigger: {}", buffer.is_completing());
        
        if buffer.is_completing() {
            let state = buffer.get_auto_completion_state();
            println!("  Prefix: '{}'", state.prefix);
            println!("  Suggestions: {}", state.items.len());
        }
        
        println!("=== Improved Trigger Test Completed ===");
    }
}