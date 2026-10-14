# vPaint MiniCa — Mini CAD cho thợ xây dựng

Ứng dụng đo vẽ mặt bằng chạy trực tiếp trong trình duyệt, hỗ trợ kéo thả, chọn/sửa đối tượng, đường thẳng, hình chữ nhật, hình tròn, đa giác, khoảng cách, tổng diện tích, chu vi, thể tích bê tông và dự toán sơ bộ. Tạo bản vẽ dưới dạng JSON, SVG, PNG hoặc in PDF bằng hộp thoại in của trình duyệt. Giao diện tiếng Việt, responsive.

**Thiết kế:** Long Ngo · **Giấy phép:** MIT (2026).

## Nền tảng
- HTML/CSS/Canvas 2D xử lý sự kiện, vẽ và UI, không cần backend.
- Rust `wasm-bindgen` tính khoảng cách, diện tích, chu vi, biên dịch thành WebAssembly. Khi chưa có WASM, Javascript fallback tính tương đương và báo trạng thái.
- GitHub Actions tự biên dịch Rust, xuất `pkg` rồi triển khai GitHub Pages.

## Chạy ở máy phát triển
```bash
cargo test
cargo install wasm-pack
wasm-pack build --target web --release --out-dir pkg
python -m http.server 8000
```
Mở http://localhost:8000; không dùng `file://` vì ES modules/WASM cần HTTP(S).

## Lưu ý kỹ thuật
Mọi tọa độ hình học đều lưu bằng **mét**; lựa chọn cm/mm chỉ thay đổi cách hiển thị. Snap gắn với lưới tính theo mét. Không sử dụng làm hồ sơ nghiệm thu/kiểm định nếu chưa đối chiếu khảo sát thực địa, tiêu chuẩn đo vẽ và bản vẽ thiết kế. Đây không phải CAD 3D hoặc bộ thay thế hoàn chỉnh DWG/DXF.