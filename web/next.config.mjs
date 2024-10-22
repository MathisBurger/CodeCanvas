/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  publicRuntimeConfig: {
    EXECUTOR_UI_URL: process.env.EXECUTOR_UI_URL,
    API_URL: process.env.API_URL,
  },
};

export default nextConfig;
