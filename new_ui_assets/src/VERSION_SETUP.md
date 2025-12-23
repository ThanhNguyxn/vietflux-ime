# VietFlux IME - Version Auto-Update Setup

## How Version Auto-Update Works

The version number is automatically read from environment variables during build time.

### Development
1. Create a `.env` file in the project root (copy from `.env.example`)
2. Set `VITE_APP_VERSION=x.y.z`
3. The version will be automatically displayed in the footer

### Production Build
The version can be injected from `package.json` during build:

```bash
# In your build script (package.json)
"scripts": {
  "build": "VITE_APP_VERSION=$(node -p \"require('./package.json').version\") vite build"
}
```

Or inject it in your CI/CD pipeline:
```bash
export VITE_APP_VERSION=$(git describe --tags --always)
npm run build
```

### How It Works
- The `FooterLinks.tsx` component reads from `import.meta.env.VITE_APP_VERSION`
- Falls back to `0.1.0` if not set
- No need to manually update version in code files
- Version is injected at build time, not runtime

### Links in Footer
- **Author**: https://github.com/ThanhNguyxn
- **Sponsor**: https://github.com/sponsors/ThanhNguyxn
- **Donate**: https://buymeacoffee.com/thanhnguyxn
- **Source**: https://github.com/ThanhNguyxn/vietflux-ime
