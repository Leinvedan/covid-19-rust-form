const gulp = require('gulp');
const htmlmin = require('gulp-htmlmin');
const cleanCSS = require('gulp-clean-css');
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
    .pipe(gulp.dest(DEST_DIR));
};

function minify_js () {
  return gulp.src(`${SOURCE}/js/*.js`)
    .pipe(minify({noSource: true}))
    .pipe(gulp.dest(DEST_DIR))
};

exports.default = gulp.series(minify_html, minify_css, minify_js);