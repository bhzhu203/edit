#[cfg(test)]
mod popup_debug_test {
    use edit::buffer::TextBuffer;
    use edit::helpers::Point;

    #[test]
    fn debug_popup_display() {
        println!("=== Debugging Popup Display ===");
        
        let mut buffer = TextBuffer::new(false).expect("Failed to create buffer");
        
        // 添加一些测试内容
        buffer.write_canon(b"hello world\nfunction test() {\n    return 42;\n}");
        
        println!("Buffer length: {} chars", buffer.text_length());
        println!("Visual lines: {}", buffer.visual_line_count());
        println!("Text width: {}", buffer.text_width());
        
        // 测试不同的光标位置
        let test_positions = vec![
            Point { x: 5, y: 0 },   // 第一行中间
            Point { x: 0, y: 1 },   // 第二行开始
            Point { x: 10, y: 2 },  // 第三行中间
        ];
        
        for (i, pos) in test_positions.iter().enumerate() {
            println!("\n--- Test {} ---", i + 1);
            buffer.cursor_move_to_logical(*pos);
            println!("Cursor at: {:?}", buffer.cursor_logical_pos());
            
            // 触发补全
            buffer.trigger_auto_completion();
            let is_active = buffer.is_completing();
            println!("Completion active: {}", is_active);
            
            if is_active {
                let state = buffer.get_auto_completion_state();
                println!("  Prefix: '{}'", state.prefix);
                println!("  Items: {}", state.items.len());
            }
        }
        
        println!("\n=== Debug Test Completed ===");
    }
}