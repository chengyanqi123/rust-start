use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.163.com".to_string());
    let smtp_user = env::var("SMTP_USER")?;
    let smtp_pass = env::var("SMTP_PASS")?;
    let smtp_port = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "465".to_string())
        .parse()?;
    let mail_to = env::var("MAIL_TO")?;

    let email = Message::builder()
        .from(smtp_user.parse()?)
        .to(mail_to.parse()?)
        .subject("Rust SMTP 测试邮件")
        .header(ContentType::TEXT_HTML)
        .body(String::from(
            r#"
<!doctype html>
<html lang="zh-CN">
  <body style="margin:0;padding:0;background:#f4f7fb;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Arial,sans-serif;color:#1f2937;">
    <div style="max-width:640px;margin:0 auto;padding:32px 16px;">
      <div style="background:#ffffff;border:1px solid #e5e7eb;border-radius:8px;overflow:hidden;">
        <div style="background:#2563eb;padding:28px 32px;color:#ffffff;">
          <h1 style="margin:0;font-size:24px;line-height:1.35;font-weight:700;">Rust SMTP 测试邮件</h1>
          <p style="margin:8px 0 0;font-size:15px;line-height:1.6;opacity:.92;">你的邮件发送功能已经成功跑起来了。</p>
        </div>

        <div style="padding:32px;">
          <p style="margin:0 0 18px;font-size:16px;line-height:1.75;">你好，</p>
          <p style="margin:0 0 18px;font-size:16px;line-height:1.75;">
            这是一封来自 Rust 程序的测试邮件。它使用 <strong>lettre</strong> 连接 SMTP 服务，并通过 HTML 正文展示更清晰的邮件样式。
          </p>

          <div style="margin:24px 0;padding:18px 20px;background:#eff6ff;border-radius:6px;">
            <p style="margin:0;font-size:15px;line-height:1.7;color:#1e3a8a;">
              如果你能看到这封邮件，说明账号配置、SMTP 连接和邮件内容构建都已经完成。
            </p>
          </div>

          <p style="margin:0 0 24px;font-size:16px;line-height:1.75;">
            接下来你可以把这里替换成验证码、通知内容、订单状态或任何业务邮件。
          </p>

          <div style="padding-top:20px;border-top:1px solid #e5e7eb;color:#6b7280;font-size:13px;line-height:1.6;">
            <p style="margin:0;">发送自 Rust mail-test 示例程序</p>
          </div>
        </div>
      </div>
    </div>
  </body>
</html>
"#,
        ))?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    let mailer = SmtpTransport::relay(&smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("邮件发送成功"),
        Err(e) => println!("邮件发送失败: {:?}", e),
    }

    Ok(())
}
