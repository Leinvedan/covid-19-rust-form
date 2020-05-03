const gulp = require('gulp');
const htmlmin = require('gulp-htmlmin');
const cleanCSS = require('gulp-clean-css');
const concat = require('gulp-concat');
const minify = require('gulp-minify');

const DEST_DIR = "./dist";
const SOURCE = "./src"


function minify_html() {
  return gulp.src(`${SOURCE}/*.html`)
    .pipe(htmlmin({ collapseWhitespace: true }))
    .pipe(gulp.dest(DEST_DIR));
};

function minify_css() {
  return gulp.src(`${SOURCE}/css/*.css`)
    .pipe(cleanCSS({compatibility: 'ie8'}))
    .pipe(concat('style.min.css'))
    .pipe(gulp.dest(DEST_DIR));
};

function minify_css_critical() {
  return gulp.src(`${SOURCE}/css/critical/*.css`)
    .pipe(cleanCSS({compatibility: 'ie8'}))
    .pipe(concat('critical.min.css'))
    .pipe(gulp.dest(DEST_DIR));
};

function minify_js () {
  return gulp.src(`${SOURCE}/js/*.js`)
    .pipe(minify({noSource: true}))
    .pipe(gulp.dest(DEST_DIR))
};

exports.default = gulp.parallel(minify_html, minify_css, minify_js, minify_css_critical);
exports.watch = () => {gulp.watch(SOURCE, gulp.parallel(minify_html, minify_css, minify_js, minify_css_critical))};