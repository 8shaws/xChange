pub fn get_email_html(otp: &str) -> String {
    format!(
        r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Email Verification</title>
                <style>
                    body {{
                        font-family: Arial, sans-serif;
                        background-color: #f4f4f4;
                        margin: 0;
                        padding: 0;
                    }}
                    .container {{
                        max-width: 600px;
                        margin: 50px auto;
                        background-color: #ffffff;
                        padding: 20px;
                        border-radius: 8px;
                        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                    }}
                    .header {{
                        text-align: center;
                        padding: 20px 0;
                        background-color: #4CAF50;
                        color: white;
                        border-radius: 8px 8px 0 0;
                    }}
                    .header h1 {{
                        margin: 0;
                    }}
                    .content {{
                        padding: 20px;
                    }}
                    .content p {{
                        font-size: 16px;
                        line-height: 1.5;
                    }}
                    .otp {{
                        display: block;
                        width: fit-content;
                        margin: 20px auto;
                        padding: 10px 20px;
                        font-size: 18px;
                        color: #4CAF50;
                        background-color: #f4f4f4;
                        border: 1px dashed #4CAF50;
                        border-radius: 4px;
                    }}
                    .button {{
                        text-align: center;
                        margin: 20px 0;
                    }}
                    .button a {{
                        text-decoration: none;
                        color: white;
                        background-color: #4CAF50;
                        padding: 10px 20px;
                        border-radius: 4px;
                    }}
                    .footer {{
                        text-align: center;
                        padding: 10px;
                        font-size: 12px;
                        color: #888888;
                    }}
                </style>
                <script>
                    function copyOTP() {{
                        var otpElement = document.getElementById('otp');
                        var range = document.createRange();
                        range.selectNode(otpElement);
                        window.getSelection().removeAllRanges();
                        window.getSelection().addRange(range);
                        try {{
                            document.execCommand('copy');
                            alert('OTP copied to clipboard');
                        }} catch(err) {{
                            console.error('Failed to copy OTP', err);
                        }}
                        window.getSelection().removeAllRanges();
                    }}
                </script>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>xChange Email Verification!</h1>
                    </div>
                    <div class="content">
                        <p>Hey There,</p>
                        <p>Thank you for registering at our exchange. Please use the OTP below to verify your email address:</p>
                        <span class="otp" id="otp" onclick="copyOTP()">{otp}</span>
                        <p>This OTP is valid for 2 minutes. If you did not request this, please ignore this email.</p>
                        <div class="button">
                            <a href="{verification_link}">Verify Email</a>
                        </div>
                        <p>Thank you,</p>
                        <p>The xChange Team</p>
                    </div>
                    <div class="footer">
                        <p>&copy; 2024 xChange. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
        otp = otp,
        verification_link = "#"
    )
}
