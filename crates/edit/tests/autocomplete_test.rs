#[cfg(test)]
mod tests {
    use edit::buffer::{TextBuffer, autocomplete::{AutoCompleter, CompletionItem}};
    use edit::helpers::Point;

    #[test]
    fn test_basic_auto_completion() {
        // Create a text buffer
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        
        // Add some text with repeated words
        buffer.write_canon(b"hello world hello test");
        
        // Move cursor to position where we can test completion
        buffer.cursor_move_to_logical(Point { x: 5, y: 0 }); // After first "hello"
        
        // Test that completion system initializes correctly
        assert!(!buffer.is_completing());
        
        // Test completion triggering
        buffer.trigger_auto_completion();
        
        // Should find "hello" as a completion for "hell"
        let is_completing = buffer.is_completing();
        println!("Is completing: {}", is_completing);
        
        if is_completing {
            let state = buffer.get_auto_completion_state();
            println!("Prefix: '{}'", state.prefix);
            println!("Items count: {}", state.items.len());
            for (i, item) in state.items.iter().enumerate() {
                println!("  {}: {}", i, item.label);
            }
            
            // Test navigation
            let initial_index = state.selected_index;
            buffer.select_next_completion();
            assert_ne!(initial_index, buffer.get_auto_completion_state().selected_index);
            
            buffer.select_prev_completion();
            assert_eq!(initial_index, buffer.get_auto_completion_state().selected_index);
        }
    }

    #[test]
    fn test_completion_acceptance() {
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        buffer.write_canon(b"test test_value");
        buffer.cursor_move_to_logical(Point { x: 4, y: 0 }); // After "test"
        
        buffer.trigger_auto_completion();
        
        if buffer.is_completing() {
            let accepted = buffer.accept_current_completion();
            // This might not accept anything since we're at the end of a word
            // but the function should not crash
            println!("Completion accepted: {}", accepted);
        }
    }

    #[test]
    fn test_completion_cancel() {
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        buffer.write_canon(b"hello world");
        buffer.cursor_move_to_logical(Point { x: 5, y: 0 });
        
        buffer.trigger_auto_completion();
        buffer.cancel_auto_completion();
        
        assert!(!buffer.is_completing());
    }
}