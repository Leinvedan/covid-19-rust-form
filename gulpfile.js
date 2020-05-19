const gulp = require('gulp');
const htmlmin = require('gulp-htmlmin');
const cleanCSS = require('gulp-clean-css');
const concat = require('gulp-concat');
const minify = require('gulp-minify');
const inject = require('gulp-inject');

const DEST_DIR = "./dist";
const SOURCE = "./src"


function minify_html() {
  let target = gulp.src(`${SOURCE}/*.html`)
    .pipe(htmlmin({ collapseWhitespace: true }))
  let critical_css = gulp.src(`${SOURCE}/css/critical/*.css`)
    .pipe(concat('critical.min.css'))
    .pipe(cleanCSS({compatibility: 'ie8'}))
  return target
      .pipe(inject(critical_css, {
          transform: function(_, file) {
            return `<style>${file.contents.toString()}</style>`
          }
        }
      ))
      .pipe(gulp.dest(DEST_DIR));
};

function minify_css() {
  return gulp.src(`${SOURCE}/css/*.css`)
    .pipe(concat('style.min.css'))
    .pipe(cleanCSS({compatibility: 'ie8'}))
    .pipe(gulp.dest(DEST_DIR));
};

function minify_js () {
  return gulp.src(`${SOURCE}/js/*.js`)
    .pipe(concat("index.js"))
    .pipe(minify({noSource: true}))
    .pipe(gulp.dest(DEST_DIR))
};

exports.default = gulp.parallel(minify_html, minify_css, minify_js);
exports.watch = () => {gulp.watch(SOURCE, gulp.parallel(minify_html, minify_css, minify_js))};