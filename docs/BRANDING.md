# ApiTap Branding & Logo Guidelines

## 🎨 Logo Overview

The ApiTap logo features a wizard character with a pointed hat and an upward arrow, symbolizing the magical transformation and upward flow of data through ETL pipelines.

### Design Philosophy

The wizard represents:
- **Magic** - Automated data transformation that "just works"
- **Wisdom** - Intelligent data processing and SQL transformations
- **Power** - Handling complex ETL pipelines effortlessly

The upward arrow represents:
- **Data Flow** - Movement from API sources to destinations
- **Transformation** - Converting raw data into valuable insights
- **Progress** - Continuous data pipeline execution

## 🎨 Color Palette

### Primary Colors

```
Navy Blue (Background)
- Hex: #0F2350
- RGB: 15, 35, 80
- Use: Primary background, dark themes

Cyan (Logo Primary)
- Hex: #5DD9E8
- RGB: 93, 217, 232
- Use: Logo outline, primary accent

Light Blue (Logo Highlight)
- Hex: #BFEEF5
- RGB: 191, 238, 245
- Use: Logo highlights, hover states

White (Text)
- Hex: #FFFFFF
- RGB: 255, 255, 255
- Use: "APITAP" text, documentation text
```

### Secondary Colors

```
Dark Blue
- Hex: #1A3A6B
- Use: Secondary backgrounds, cards

Medium Blue
- Hex: #2D5A9C
- Use: Links, interactive elements
```

## 📐 Logo Variations

### 1. Full Logo (Recommended)
- Wizard icon + "APITAP" text
- Use: README, documentation headers, presentations
- Minimum width: 200px
- Aspect ratio: Maintain original proportions

### 2. Icon Only
- Wizard icon without text
- Use: Favicons, app icons, social media avatars
- Sizes: 16x16, 32x32, 64x64, 128x128, 256x256, 512x512
- Format: PNG with transparency, ICO for Windows

### 3. Light Background Variant
- Darker wizard outline for light backgrounds
- Use: Light-themed documentation, presentations with white backgrounds
- Background: #FFFFFF or light colors

### 4. Monochrome
- Single color version for limited color contexts
- Use: Print materials, stamps, watermarks
- Color: Black (#000000) or White (#FFFFFF)

## 📏 Clear Space & Sizing

### Minimum Clear Space
Maintain a clear space around the logo equal to the height of the "A" in "APITAP" on all sides.

### Minimum Sizes
- **Digital**: 120px width (with text), 32px width (icon only)
- **Print**: 1 inch width (with text), 0.25 inch width (icon only)

### Maximum Sizes
- No maximum, maintain aspect ratio
- Ensure crisp rendering at all sizes

## ✅ Logo Usage Guidelines

### DO:
✅ Use official logo files provided
✅ Maintain aspect ratio when scaling
✅ Use on contrasting backgrounds for visibility
✅ Provide adequate clear space around logo
✅ Use PNG format for digital applications
✅ Use SVG format for web and scalable needs
✅ Use appropriate variant for context (light/dark)

### DON'T:
❌ Distort, stretch, or skew the logo
❌ Change logo colors outside brand palette
❌ Add effects (shadows, glows, outlines)
❌ Rotate the logo
❌ Place on busy or low-contrast backgrounds
❌ Modify the wizard design
❌ Use low-resolution versions
❌ Recreate or redraw the logo

## 📁 Logo File Formats

### Required Formats

```
logo/
├── apitap-logo.svg          # Vector (preferred for web)
├── apitap-logo.png          # Raster with transparency
├── apitap-logo-512.png      # 512x512px
├── apitap-logo-256.png      # 256x256px
├── apitap-logo-128.png      # 128x128px
├── apitap-logo-64.png       # 64x64px
├── apitap-logo-32.png       # 32x32px
├── apitap-logo-16.png       # 16x16px (favicon)
├── apitap-icon-only.svg     # Icon without text
├── apitap-icon-only.png     # Icon PNG
├── apitap-logo-light.svg    # Light background variant
├── apitap-logo-mono.svg     # Monochrome version
└── favicon.ico              # Windows icon format
```

## 🖼️ Usage Examples

### README.md Header
```markdown
<p align="center">
  <img src="docs/logo/apitap-logo.png" alt="ApiTap Logo" width="300">
</p>

# ApiTap
*Magical ETL Pipelines Made Simple*
```

### Documentation Site
```html
<div class="header">
  <img src="/logo/apitap-logo.svg" alt="ApiTap" height="60">
</div>
```

### Social Media Profile
- Use: Icon-only version
- Size: 400x400px minimum
- Format: PNG with transparency
- Background: Navy blue (#0F2350)

### GitHub Repository
- Social preview: 1280x640px with full logo
- Avatar: Icon-only, 256x256px

## 🎯 Brand Voice

When representing ApiTap, maintain:
- **Professional yet approachable** - Like a helpful wizard
- **Technical but clear** - Expert without jargon
- **Magical** - Data transformation made effortless
- **Reliable** - Production-ready, battle-tested

## 📝 Typography

### Primary Font
- **Logo**: Bold sans-serif (as in current design)
- **Headings**: System fonts (Inter, Roboto, system-ui)
- **Body**: System fonts for readability
- **Code**: Monospace (Fira Code, JetBrains Mono, Consolas)

### Text Styling
```css
/* Logo Text */
.logo-text {
  font-weight: 700;
  letter-spacing: 0.05em;
  color: #FFFFFF;
}

/* Tagline */
.tagline {
  font-weight: 400;
  font-style: italic;
  color: #5DD9E8;
}
```

## 🌐 Digital Applications

### Website Favicon
```html
<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
<link rel="shortcut icon" href="/favicon.ico">
```

### Open Graph / Social Media
```html
<!-- Open Graph -->
<meta property="og:image" content="https://yourdomain.com/logo/apitap-og-image.png">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">

<!-- Twitter Card -->
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:image" content="https://yourdomain.com/logo/apitap-twitter-card.png">
```

### GitHub Repository Settings
- Repository image: 1280x640px with logo centered
- Profile picture: Icon-only, 256x256px or larger
- Use PNG format with transparency

## 📦 Package Managers

### Cargo/crates.io
Add logo to `Cargo.toml`:
```toml
[package]
name = "apitap"
# ... other fields
repository = "https://github.com/yourusername/apitap"
# Logo will be displayed from repository
```

### npm (if applicable)
```json
{
  "name": "apitap",
  "icon": "logo/apitap-icon-only.png"
}
```

## 🎬 Presentation & Marketing

### Slide Decks
- Use full logo on title slide
- Use icon-only in header/footer
- Maintain minimum 200px width for readability
- Use navy blue background for consistency

### Blog Posts & Articles
- Feature image: 1200x630px with logo
- Inline logo: 150-200px width
- Use icon as bullet points for ApiTap features

### Video Thumbnails
- Resolution: 1280x720px minimum
- Logo placement: Center or bottom-right
- Include tagline: "Magical ETL Pipelines"

## 🔧 Developer Resources

### CLI Tool
```rust
// Display ASCII art logo in terminal
const LOGO_ASCII: &str = r#"
    🧙‍♂️
   APITAP
ETL Pipeline Tool
"#;
```

### Loading Animations
Use wizard hat or wand emoji: 🧙‍♂️ 🪄 ⬆️

### Error Messages
Maintain brand voice:
```
❌ Oops! The magic failed...
✨ Pipeline completed successfully!
🔮 Fetching data from API...
```

## 📄 License & Attribution

### Logo Copyright
© 2025 ApiTap. All rights reserved.

### Usage Permission
- ✅ Open source projects using ApiTap
- ✅ Blog posts & tutorials about ApiTap
- ✅ Educational & non-commercial use
- ❌ Commercial products without permission
- ❌ Modification or derivative works

### Attribution
When featuring ApiTap:
```
"ApiTap logo used with permission"
or
"Powered by ApiTap"
```

## 🆘 Logo Support

### Need Help?
- **Vector files**: Contact for SVG/AI source files
- **Custom sizes**: Request specific dimensions
- **Brand questions**: Reach out for clarification
- **Violations**: Report misuse

### Contact
- GitHub Issues: For technical questions
- Email: For branding inquiries

---

## 📋 Quick Reference

| Use Case | Variant | Size | Format |
|----------|---------|------|--------|
| README Header | Full Logo | 300px width | PNG/SVG |
| Favicon | Icon Only | 32x32px | ICO/PNG |
| Social Media | Icon Only | 400x400px | PNG |
| Documentation | Full Logo | 200-400px | SVG |
| Print | Full Logo | 2" width min | PDF/SVG |
| App Icon | Icon Only | 512x512px | PNG |
| Email Signature | Full Logo | 150px width | PNG |

## 🎨 Design Assets Checklist

Before launching, ensure you have:

- [ ] SVG logo (full version)
- [ ] SVG logo (icon only)
- [ ] PNG logos in all standard sizes (16-512px)
- [ ] Favicon.ico file
- [ ] Light background variant
- [ ] Monochrome version
- [ ] Social media images (1200x630px)
- [ ] GitHub social preview image
- [ ] Press kit with all variants
- [ ] Brand guidelines PDF

---

*Last Updated: December 2025*
*Version: 1.0*
