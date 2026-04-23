import AppKit
import CoreGraphics
import Foundation

// Renders a 1024x1024 PNG for AutoClicker: rounded-square blue→purple
// gradient background with a white target reticle (ring + crosshair + dot).

let size = 1024
let ctx = CGContext(
    data: nil,
    width: size,
    height: size,
    bitsPerComponent: 8,
    bytesPerRow: 0,
    space: CGColorSpace(name: CGColorSpace.sRGB)!,
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
)!

let rect = CGRect(x: 0, y: 0, width: size, height: size)
let corner: CGFloat = CGFloat(size) * 0.225 // macOS "squircle"-ish radius

// Rounded-square clip
let path = CGPath(roundedRect: rect, cornerWidth: corner, cornerHeight: corner, transform: nil)
ctx.addPath(path)
ctx.clip()

// Gradient background (blue → purple, top-left → bottom-right)
let colors = [
    CGColor(red: 0.29, green: 0.51, blue: 0.98, alpha: 1.0),
    CGColor(red: 0.54, green: 0.27, blue: 0.91, alpha: 1.0)
] as CFArray
let gradient = CGGradient(colorsSpace: nil, colors: colors, locations: [0, 1])!
ctx.drawLinearGradient(
    gradient,
    start: CGPoint(x: 0, y: CGFloat(size)),
    end: CGPoint(x: CGFloat(size), y: 0),
    options: []
)

// Subtle top highlight
let hl = [
    CGColor(red: 1, green: 1, blue: 1, alpha: 0.18),
    CGColor(red: 1, green: 1, blue: 1, alpha: 0.0)
] as CFArray
let hlg = CGGradient(colorsSpace: nil, colors: hl, locations: [0, 1])!
ctx.drawLinearGradient(
    hlg,
    start: CGPoint(x: 0, y: CGFloat(size)),
    end: CGPoint(x: 0, y: CGFloat(size) * 0.55),
    options: []
)

// Target reticle (white)
let cx = CGFloat(size) / 2
let cy = CGFloat(size) / 2
let white = CGColor(red: 1, green: 1, blue: 1, alpha: 1)
ctx.setStrokeColor(white)
ctx.setFillColor(white)
ctx.setLineCap(.round)

// Outer ring
let ringR: CGFloat = CGFloat(size) * 0.30
let ringW: CGFloat = CGFloat(size) * 0.05
ctx.setLineWidth(ringW)
ctx.strokeEllipse(in: CGRect(x: cx - ringR, y: cy - ringR, width: ringR * 2, height: ringR * 2))

// Crosshair gaps: 4 tick marks pointing inward
let tickOuter: CGFloat = ringR + CGFloat(size) * 0.09
let tickInner: CGFloat = ringR + CGFloat(size) * 0.015
let tickW: CGFloat = CGFloat(size) * 0.05
ctx.setLineWidth(tickW)

let dirs: [(CGFloat, CGFloat)] = [(1, 0), (-1, 0), (0, 1), (0, -1)]
for (dx, dy) in dirs {
    ctx.move(to: CGPoint(x: cx + dx * tickInner, y: cy + dy * tickInner))
    ctx.addLine(to: CGPoint(x: cx + dx * tickOuter, y: cy + dy * tickOuter))
    ctx.strokePath()
}

// Center dot
let dotR: CGFloat = CGFloat(size) * 0.065
ctx.fillEllipse(in: CGRect(x: cx - dotR, y: cy - dotR, width: dotR * 2, height: dotR * 2))

// Write PNG
guard let cgImage = ctx.makeImage() else {
    FileHandle.standardError.write("failed to make image\n".data(using: .utf8)!)
    exit(1)
}
let rep = NSBitmapImageRep(cgImage: cgImage)
rep.size = NSSize(width: size, height: size)
guard let data = rep.representation(using: .png, properties: [:]) else {
    FileHandle.standardError.write("failed to encode png\n".data(using: .utf8)!)
    exit(1)
}
let outPath = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "icon_1024.png"
try data.write(to: URL(fileURLWithPath: outPath))
print("wrote \(outPath)")
