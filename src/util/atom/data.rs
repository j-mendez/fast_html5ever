/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use phf::{PhfSet, PhfMap};

pub static atoms: PhfSet = phf_set!(
    "",
    "a",
    "abbr",
    "abs",
    "accent",
    "accent-height",
    "accentunder",
    "accept",
    "accept-charset",
    "accesskey",
    "accumulate",
    "acronym",
    "action",
    "actiontype",
    "active",
    "actuate",
    "additive",
    "address",
    "align",
    "alignment-baseline",
    "alignmentscope",
    "alink",
    "alphabetic",
    "alt",
    "altglyph",
    "altGlyph",
    "altglyphdef",
    "altGlyphDef",
    "altglyphitem",
    "altGlyphItem",
    "altimg",
    "alttext",
    "amplitude",
    "and",
    "animate",
    "animatecolor",
    "animateColor",
    "animatemotion",
    "animateMotion",
    "animatetransform",
    "animateTransform",
    "animation",
    "annotation",
    "annotation-xml",
    "applet",
    "apply",
    "approx",
    "arabic-form",
    "arccos",
    "arccosh",
    "arccot",
    "arccoth",
    "arccsc",
    "arccsch",
    "archive",
    "arcrole",
    "arcsec",
    "arcsech",
    "arcsin",
    "arcsinh",
    "arctan",
    "arctanh",
    "area",
    "arg",
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-channel",
    "aria-checked",
    "aria-controls",
    "aria-datatype",
    "aria-describedby",
    "aria-disabled",
    "aria-dropeffect",
    "aria-expanded",
    "aria-flowto",
    "aria-grab",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-multiline",
    "aria-multiselectable",
    "aria-owns",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-secret",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-templateid",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "article",
    "ascent",
    "aside",
    "async",
    "attributename",
    "attributeName",
    "attributetype",
    "attributeType",
    "audio",
    "autocomplete",
    "autofocus",
    "autoplay",
    "autosubmit",
    "axis",
    "azimuth",
    "b",
    "background",
    "base",
    "basefont",
    "basefrequency",
    "baseFrequency",
    "baseline",
    "baseline-shift",
    "baseprofile",
    "baseProfile",
    "bbox",
    "bdo",
    "begin",
    "bevelled",
    "bgcolor",
    "bgsound",
    "bias",
    "big",
    "blockquote",
    "body",
    "border",
    "br",
    "button",
    "bvar",
    "by",
    "calcmode",
    "calcMode",
    "canvas",
    "cap-height",
    "caption",
    "card",
    "cartesianproduct",
    "ceiling",
    "cellpadding",
    "cellspacing",
    "center",
    "char",
    "charoff",
    "charset",
    "checked",
    "ci",
    "circle",
    "cite",
    "class",
    "classid",
    "clear",
    "clip",
    "clippath",
    "clip-path",
    "clipPath",
    "clippathunits",
    "clipPathUnits",
    "clip-rule",
    "close",
    "closure",
    "cn",
    "code",
    "codebase",
    "codetype",
    "codomain",
    "col",
    "colgroup",
    "color",
    "color-interpolation",
    "color-interpolation-filters",
    "color-profile",
    "color-rendering",
    "cols",
    "colspan",
    "columnalign",
    "columnlines",
    "columnspacing",
    "columnspan",
    "columnwidth",
    "compact",
    "complexes",
    "compose",
    "condition",
    "conjugate",
    "content",
    "contenteditable",
    "contentscripttype",
    "contentScriptType",
    "contentstyletype",
    "contentStyleType",
    "contextmenu",
    "controls",
    "coords",
    "cos",
    "cosh",
    "cot",
    "coth",
    "crossorigin",
    "csc",
    "csch",
    "csymbol",
    "curl",
    "cursor",
    "cx",
    "cy",
    "d",
    "data",
    "datafld",
    "dataformatas",
    "datasrc",
    "datatemplate",
    "datetime",
    "dd",
    "declare",
    "default",
    "defer",
    "definition-src",
    "definitionurl",
    "definitionURL",
    "defs",
    "degree",
    "del",
    "depth",
    "desc",
    "descent",
    "details",
    "determinant",
    "dfn",
    "diff",
    "diffuseconstant",
    "diffuseConstant",
    "dir",
    "direction",
    "disabled",
    "discard",
    "display",
    "displaystyle",
    "div",
    "divergence",
    "divide",
    "divisor",
    "dl",
    "domain",
    "domainofapplication",
    "dominant-baseline",
    "draggable",
    "dt",
    "dur",
    "dx",
    "dy",
    "edge",
    "edgemode",
    "edgeMode",
    "elevation",
    "ellipse",
    "em",
    "embed",
    "emptyset",
    "enable-background",
    "encoding",
    "enctype",
    "end",
    "eq",
    "equalcolumns",
    "equalrows",
    "equivalent",
    "eulergamma",
    "exists",
    "exp",
    "exponent",
    "exponentiale",
    "externalresourcesrequired",
    "externalResourcesRequired",
    "face",
    "factorial",
    "factorof",
    "false",
    "feblend",
    "feBlend",
    "fecolormatrix",
    "feColorMatrix",
    "fecomponenttransfer",
    "feComponentTransfer",
    "fecomposite",
    "feComposite",
    "feconvolvematrix",
    "feConvolveMatrix",
    "fediffuselighting",
    "feDiffuseLighting",
    "fedisplacementmap",
    "feDisplacementMap",
    "fedistantlight",
    "feDistantLight",
    "feflood",
    "feFlood",
    "fefunca",
    "feFuncA",
    "fefuncb",
    "feFuncB",
    "fefuncg",
    "feFuncG",
    "fefuncr",
    "feFuncR",
    "fegaussianblur",
    "feGaussianBlur",
    "feimage",
    "feImage",
    "femerge",
    "feMerge",
    "femergenode",
    "feMergeNode",
    "femorphology",
    "feMorphology",
    "fence",
    "feoffset",
    "feOffset",
    "fepointlight",
    "fePointLight",
    "fespecularlighting",
    "feSpecularLighting",
    "fespotlight",
    "feSpotLight",
    "fetile",
    "feTile",
    "feturbulence",
    "feTurbulence",
    "fieldset",
    "figcaption",
    "figure",
    "fill",
    "fill-opacity",
    "fill-rule",
    "filter",
    "filterres",
    "filterRes",
    "filterunits",
    "filterUnits",
    "flood-color",
    "flood-opacity",
    "floor",
    "fn",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "fontfamily",
    "font-family",
    "fontsize",
    "font-size",
    "font-size-adjust",
    "font-stretch",
    "fontstyle",
    "font-style",
    "font-variant",
    "fontweight",
    "font-weight",
    "footer",
    "for",
    "forall",
    "foreignobject",
    "foreignObject",
    "form",
    "format",
    "frame",
    "frameborder",
    "frameset",
    "framespacing",
    "from",
    "fx",
    "fy",
    "g",
    "g1",
    "g2",
    "gcd",
    "geq",
    "glyph",
    "glyph-name",
    "glyph-orientation-horizontal",
    "glyph-orientation-vertical",
    "glyphref",
    "glyphRef",
    "grad",
    "gradienttransform",
    "gradientTransform",
    "gradientunits",
    "gradientUnits",
    "groupalign",
    "gt",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "handler",
    "hanging",
    "head",
    "header",
    "headers",
    "height",
    "hgroup",
    "hidden",
    "hidefocus",
    "high",
    "hkern",
    "horiz-adv-x",
    "horiz-origin-x",
    "horiz-origin-y",
    "hr",
    "href",
    "hreflang",
    "hspace",
    "html",
    "http-equiv",
    "i",
    "icon",
    "id",
    "ident",
    "ideographic",
    "iframe",
    "image",
    "image-rendering",
    "imaginary",
    "imaginaryi",
    "img",
    "implies",
    "in",
    "in2",
    "index",
    "infinity",
    "input",
    "inputmode",
    "ins",
    "int",
    "integers",
    "intercept",
    "intersect",
    "interval",
    "inverse",
    "irrelevant",
    "isindex",
    "ismap",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kbd",
    "kernelmatrix",
    "kernelMatrix",
    "kernelunitlength",
    "kernelUnitLength",
    "kerning",
    "keygen",
    "keypoints",
    "keyPoints",
    "keysplines",
    "keySplines",
    "keytimes",
    "keyTimes",
    "label",
    "lambda",
    "lang",
    "language",
    "laplacian",
    "largeop",
    "lcm",
    "legend",
    "lengthadjust",
    "lengthAdjust",
    "leq",
    "letter-spacing",
    "li",
    "lighting-color",
    "limit",
    "limitingconeangle",
    "limitingConeAngle",
    "line",
    "lineargradient",
    "linearGradient",
    "linebreak",
    "linethickness",
    "link",
    "list",
    "listener",
    "listing",
    "ln",
    "local",
    "log",
    "logbase",
    "longdesc",
    "loop",
    "low",
    "lowlimit",
    "lowsrc",
    "lquote",
    "lspace",
    "lt",
    "macros",
    "maction",
    "main",
    "maligngroup",
    "malignmark",
    "manifest",
    "map",
    "marginheight",
    "marginwidth",
    "mark",
    "marker",
    "marker-end",
    "markerheight",
    "markerHeight",
    "marker-mid",
    "marker-start",
    "markerunits",
    "markerUnits",
    "markerwidth",
    "markerWidth",
    "marquee",
    "mask",
    "maskcontentunits",
    "maskContentUnits",
    "maskunits",
    "maskUnits",
    "math",
    "mathbackground",
    "mathcolor",
    "mathematical",
    "mathsize",
    "mathvariant",
    "matrix",
    "matrixrow",
    "max",
    "maxlength",
    "maxsize",
    "mean",
    "media",
    "median",
    "mediummathspace",
    "menclose",
    "menu",
    "menuitem",
    "merror",
    "meta",
    "metadata",
    "meter",
    "method",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "min",
    "minsize",
    "minus",
    "missing-glyph",
    "mlabeledtr",
    "mmultiscripts",
    "mn",
    "mo",
    "mode",
    "moment",
    "momentabout",
    "movablelimits",
    "mover",
    "mpadded",
    "mpath",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "multiple",
    "munder",
    "munderover",
    "name",
    "nargs",
    "naturalnumbers",
    "nav",
    "neq",
    "nest",
    "nobr",
    "noembed",
    "noframes",
    "nohref",
    "none",
    "noresize",
    "noscript",
    "noshade",
    "not",
    "notanumber",
    "notation",
    "notin",
    "notprsubset",
    "notsubset",
    "nowrap",
    "numoctaves",
    "numOctaves",
    "object",
    "occurrence",
    "offset",
    "ol",
    "onabort",
    "onactivate",
    "onafterprint",
    "onafterupdate",
    "onbefordeactivate",
    "onbeforeactivate",
    "onbeforecopy",
    "onbeforecut",
    "onbeforeeditfocus",
    "onbeforepaste",
    "onbeforeprint",
    "onbeforeunload",
    "onbeforeupdate",
    "onbegin",
    "onblur",
    "onbounce",
    "oncellchange",
    "onchange",
    "onclick",
    "oncontextmenu",
    "oncontrolselect",
    "oncopy",
    "oncut",
    "ondataavailable",
    "ondatasetchanged",
    "ondatasetcomplete",
    "ondblclick",
    "ondeactivate",
    "ondrag",
    "ondragdrop",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "onend",
    "onerror",
    "onerrorupdate",
    "onfilterchange",
    "onfinish",
    "onfocus",
    "onfocusin",
    "onfocusout",
    "onformchange",
    "onforminput",
    "onhelp",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onload",
    "onlosecapture",
    "onmessage",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onmousewheel",
    "onmove",
    "onmoveend",
    "onmovestart",
    "onpaste",
    "onpropertychange",
    "onreadystatechange",
    "onrepeat",
    "onreset",
    "onresize",
    "onrowenter",
    "onrowexit",
    "onrowsdelete",
    "onrowsinserted",
    "onscroll",
    "onselect",
    "onselectstart",
    "onstart",
    "onstop",
    "onsubmit",
    "onunload",
    "onzoom",
    "opacity",
    "open",
    "operator",
    "optgroup",
    "optimum",
    "option",
    "or",
    "order",
    "orient",
    "orientation",
    "origin",
    "other",
    "otherwise",
    "outerproduct",
    "output",
    "overflow",
    "overline-position",
    "overline-thickness",
    "p",
    "panose-1",
    "param",
    "partialdiff",
    "path",
    "pathlength",
    "pathLength",
    "pattern",
    "patterncontentunits",
    "patternContentUnits",
    "patterntransform",
    "patternTransform",
    "patternunits",
    "patternUnits",
    "pi",
    "piece",
    "piecewise",
    "ping",
    "plaintext",
    "plus",
    "pointer-events",
    "points",
    "pointsatx",
    "pointsAtX",
    "pointsaty",
    "pointsAtY",
    "pointsatz",
    "pointsAtZ",
    "polygon",
    "polyline",
    "poster",
    "power",
    "pre",
    "prefetch",
    "preservealpha",
    "preserveAlpha",
    "preserveaspectratio",
    "preserveAspectRatio",
    "primes",
    "primitiveunits",
    "primitiveUnits",
    "product",
    "profile",
    "progress",
    "prompt",
    "prsubset",
    "q",
    "quotient",
    "r",
    "radialgradient",
    "radialGradient",
    "radiogroup",
    "radius",
    "rationals",
    "readonly",
    "real",
    "reals",
    "rect",
    "refx",
    "refX",
    "refy",
    "refY",
    "rel",
    "reln",
    "rem",
    "rendering-intent",
    "repeat",
    "repeatcount",
    "repeatCount",
    "repeatdur",
    "repeatDur",
    "repeat-max",
    "repeat-min",
    "repeat-start",
    "repeat-template",
    "replace",
    "required",
    "requiredextensions",
    "requiredExtensions",
    "requiredfeatures",
    "requiredFeatures",
    "restart",
    "result",
    "rev",
    "role",
    "root",
    "rotate",
    "rowalign",
    "rowlines",
    "rows",
    "rowspacing",
    "rowspan",
    "rp",
    "rquote",
    "rspace",
    "rt",
    "ruby",
    "rule",
    "rules",
    "rx",
    "ry",
    "s",
    "samp",
    "sandbox",
    "scalarproduct",
    "scale",
    "scheme",
    "scope",
    "scoped",
    "script",
    "scriptlevel",
    "scriptminsize",
    "scriptsizemultiplier",
    "scrolldelay",
    "scrolling",
    "sdev",
    "seamless",
    "sec",
    "sech",
    "section",
    "seed",
    "select",
    "selected",
    "selection",
    "selector",
    "semantics",
    "sep",
    "separator",
    "separators",
    "set",
    "setdiff",
    "shape",
    "shape-rendering",
    "show",
    "sin",
    "sinh",
    "size",
    "slope",
    "small",
    "solidcolor",
    "source",
    "space",
    "spacing",
    "span",
    "specification",
    "specularconstant",
    "specularConstant",
    "specularexponent",
    "specularExponent",
    "speed",
    "spreadmethod",
    "spreadMethod",
    "src",
    "srcdoc",
    "standby",
    "start",
    "startoffset",
    "startOffset",
    "stddeviation",
    "stdDeviation",
    "stemh",
    "stemv",
    "step",
    "stitchtiles",
    "stitchTiles",
    "stop",
    "stop-color",
    "stop-opacity",
    "stretchy",
    "strike",
    "strikethrough-position",
    "strikethrough-thickness",
    "string",
    "stroke",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-opacity",
    "stroke-width",
    "strong",
    "style",
    "sub",
    "subscriptshift",
    "subset",
    "sum",
    "summary",
    "sup",
    "superscriptshift",
    "surfacescale",
    "surfaceScale",
    "svg",
    "switch",
    "symbol",
    "symmetric",
    "systemlanguage",
    "systemLanguage",
    "tabindex",
    "table",
    "tablevalues",
    "tableValues",
    "tan",
    "tanh",
    "target",
    "targetx",
    "targetX",
    "targety",
    "targetY",
    "tbody",
    "tbreak",
    "td",
    "template",
    "tendsto",
    "text",
    "text-anchor",
    "textarea",
    "text-decoration",
    "textlength",
    "textLength",
    "textpath",
    "textPath",
    "text-rendering",
    "tfoot",
    "th",
    "thead",
    "thickmathspace",
    "thinmathspace",
    "time",
    "times",
    "title",
    "to",
    "tr",
    "track",
    "transform",
    "transpose",
    "tref",
    "true",
    "tspan",
    "tt",
    "type",
    "u",
    "u1",
    "u2",
    "ul",
    "underline-position",
    "underline-thickness",
    "unicode",
    "unicode-bidi",
    "unicode-range",
    "union",
    "units-per-em",
    "unselectable",
    "uplimit",
    "use",
    "usemap",
    "valign",
    "v-alphabetic",
    "value",
    "values",
    "valuetype",
    "var",
    "variance",
    "vector",
    "vectorproduct",
    "version",
    "vert-adv-y",
    "vert-origin-x",
    "vert-origin-y",
    "verythickmathspace",
    "verythinmathspace",
    "veryverythickmathspace",
    "veryverythinmathspace",
    "v-hanging",
    "video",
    "v-ideographic",
    "view",
    "viewbox",
    "viewBox",
    "viewtarget",
    "viewTarget",
    "visibility",
    "vkern",
    "vlink",
    "v-mathematical",
    "vspace",
    "wbr",
    "when",
    "width",
    "widths",
    "word-spacing",
    "wrap",
    "writing-mode",
    "x",
    "x1",
    "x2",
    "xchannelselector",
    "xChannelSelector",
    "x-height",
    "xlink:actuate",
    "xlink:arcrole",
    "xlink:href",
    "xlink:role",
    "xlink:show",
    "xlink:type",
    "xml:base",
    "xml:lang",
    "xmlns",
    "xmlns:xlink",
    "xml:space",
    "xmp",
    "xor",
    "xref",
    "y",
    "y1",
    "y2",
    "ychannelselector",
    "yChannelSelector",
    "z",
    "zoomandpan",
    "zoomAndPan",
);
