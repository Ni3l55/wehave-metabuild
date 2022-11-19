/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
  images: { domains: ['ipfs.io'], formats: ['image/avif', 'image/webp'], },
}

module.exports = nextConfig
