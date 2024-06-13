use super::{Chunk, ChunkVariant};

pub struct Paragraph;

impl Paragraph {
    pub fn from_chunks(chunks: Vec<Chunk>) -> Vec<Chunk> {
        let mut new = Vec::new();

        for chunk in chunks {
            let mut line_no = chunk.line_no();
            let mut text_buffer = String::new();
            let mut text_buffer_line = 1;
            let mut buffer = String::new();
            let mut buffer_line = 1;
            let mut consec_newlines = 0;

            if let ChunkVariant::Text(s) = chunk.variant() {
                for c in s.chars() {
                    match c {
                        '\n' if consec_newlines == 1 => {
                            consec_newlines += 1;
                            line_no += 1;

                            if !text_buffer.is_empty() {
                                new.push(Chunk::new(
                                    text_buffer_line,
                                    ChunkVariant::Text(std::mem::take(&mut text_buffer)),
                                ));
                            }

                            buffer.push(c);
                        }
                        '\n' => {
                            buffer_line = line_no;
                            consec_newlines += 1;
                            line_no += 1;

                            buffer.push(c);
                        }
                        _ if c.is_whitespace() && consec_newlines == 0 => text_buffer.push(c),
                        _ if c.is_whitespace() => buffer.push(c),
                        _ if consec_newlines == 0 => {
                            if text_buffer.is_empty() {
                                text_buffer_line = line_no;
                            }

                            text_buffer.push(c)
                        }
                        _ if consec_newlines == 1 => {
                            if text_buffer.is_empty() {
                                text_buffer_line = buffer_line;
                            }

                            text_buffer.push_str(&std::mem::take(&mut buffer));
                            text_buffer.push(c);
                            consec_newlines = 0;
                        }
                        _ => {
                            new.push(Chunk::new(
                                buffer_line,
                                ChunkVariant::ParagraphBreak(std::mem::take(&mut buffer)),
                            ));

                            text_buffer_line = line_no;
                            consec_newlines = 0;

                            text_buffer.push(c)
                        }
                    }
                }

                if !buffer.is_empty() {
                    if consec_newlines > 1 {
                        new.push(Chunk::new(
                            buffer_line,
                            ChunkVariant::ParagraphBreak(std::mem::take(&mut buffer)),
                        ));
                    } else {
                        text_buffer.push_str(&buffer);

                        if !text_buffer.is_empty() {
                            new.push(Chunk::new(
                                text_buffer_line,
                                ChunkVariant::Text(text_buffer),
                            ))
                        }
                    }
                }
            } else {
                new.push(chunk);
            }
        }

        new
    }
}
