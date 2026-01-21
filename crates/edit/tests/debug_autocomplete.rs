#[cfg(test)]
mod debug_tests {
    use edit::buffer::{TextBuffer, autocomplete::AutoCompleter};
    use edit::helpers::Point;

    #[test]
    fn debug_completion_triggering() {
        // Redirect stdout to see the output
        use std::io::Write;
        
        // Create a text buffer with test content
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        
        // Add content with repeated words
        buffer.write_canon(b"hello world hello test function hello_world");
        
        println!("=== Auto-completion Debug Test ===");
        println!("Buffer content: \"hello world hello test function hello_world\"");
        println!("Buffer length: {}", buffer.text_length());
        
        // Test 1: Move cursor and trigger completion manually
        buffer.cursor_move_to_logical(Point { x: 5, y: 0 }); // Position after first "hello"
        println!("Cursor position: {:?}", buffer.cursor_logical_pos());
        
        // Test manual triggering
        buffer.trigger_auto_completion();
        println!("Manual trigger result - is completing: {}", buffer.is_completing());
        
        if buffer.is_completing() {
            let state = buffer.get_auto_completion_state();
            println!("  Prefix found: '{}'", state.prefix);
            println!("  Items found: {}", state.items.len());
            for item in &state.items {
                println!("    - {}", item.label);
            }
        } else {
            println!("  No completions found");
            
            // Debug: Check what words are actually in the buffer
            println!("  Checking buffer content for words:");
            let completions_debug = buffer.get_completions_for_prefix("");
            println!("  Total words found: {}", completions_debug.len());
            for (i, item) in completions_debug.iter().enumerate().take(10) {
                println!("    {}: {}", i, item.label);
            }
        }
        
        // Test 2: Check word extraction directly
        println!("\nDirect word extraction test:");
        let completions = buffer.get_completions_for_prefix("hel");
        println!("Words starting with 'hel': {}", completions.len());
        for item in completions {
            println!("  - {}", item.label);
        }
        
        // Test 3: Try with different prefix
        let completions2 = buffer.get_completions_for_prefix("hello");
        println!("\nWords starting with 'hello': {}", completions2.len());
        for item in completions2 {
            println!("  - {}", item.label);
        }
        
        println!("=== Debug Test Completed ===");
    }
}