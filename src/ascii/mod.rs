use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Error {
    Int(ParseIntError),
    Unicode(u32),
}

fn parse_unicode(input: &str) -> Result<char, Error> {
    let unicode = u32::from_str_radix(input, 10).map_err(Error::Int)?;
    char::from_u32(unicode).ok_or_else(|| Error::Unicode(unicode))
}

/// `decode` 将包含 HTML 实体编码的字符串转换为其对应的原始字符。
///
/// # 说明
///
/// 这个函数接受一个包含 HTML 实体编码的字符串，并将其中的每个编码（如 `&#27979;`）转换为其对应的 Unicode 字符。函数假设所有的编码都以 `&#` 开始，并以 `;` 结束。若遇到解析错误，则将错误的部分转换为空字符串。
///
/// # 示例
///
/// ```rust
/// # use utils_rust::ascii::decode;
/// assert_eq!( decode("&#27979;&#35797;"), "测试");
/// ```
///
/// # 注意事项
///
/// - 这个函数假设输入的 HTML 实体编码字符串中的所有编码都是有效的，并且以 `&#` 开始且以 `;` 结束。
/// - 对于无法解析的编码，函数将返回空字符串。
pub fn decode(u: &str) -> String {
    u.split(';')
        .map(|item| {
            let u = item.replace("&#", "");
            match parse_unicode(&u) {
                Ok(x) => x.to_string(),
                Err(_) => "".to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

/// 将字符串编码为 HTML 实体编码格式。
///
/// # 说明
///
/// 这个函数将输入字符串中的每个字符转换为其对应的 HTML 实体编码。每个字符都会被转换为类似 `&#1234;` 的格式，其中 `1234` 是字符的 Unicode 代码点的十进制表示。
///
/// # 示例
///
/// ```rust
/// # use utils_rust::ascii::encode;
/// assert_eq!( encode("测试"), "&#27979;&#35797;");
/// ```
///
/// # 注意事项
///
/// - 函数对每个字符进行编码，不管字符是否是 ASCII 还是非 ASCII 字符。
/// - 编码结果是一个包含 HTML 实体编码的字符串，可以在 HTML 文档中直接使用。
pub fn encode(s: &str) -> String {
    s.chars()
        .map(|c| format!("&#{};", c as u32))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let a = "测试".to_string();
        let b = "&#27979;&#35797;";
        let c = decode(b);
        assert_eq!(a, c)
    }

    #[test]
    fn test_encode() {
        let a = "测试";
        let b = "&#27979;&#35797;".to_string();
        let c = encode(a);
        assert_eq!(b, c)
    }

    #[test]
    fn test_parse_unicode() {
        assert_eq!(parse_unicode("128077"), Ok('👍'));
    }
}
