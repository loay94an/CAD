# CADCraft Android — التدقيق الوظيفي الموسع وخطة التنفيذ

**المستودع:** loay94an/CAD  
**الفرع:** feat/android-phone-tablet  
**تاريخ التدقيق:** 2026-11-24  
**المرجع:** 87b2070a04377462117d5e273c1c84b646ebff75 (ثم يثبت commit هذا الملف بعد الحفظ)  
**نوع التدقيق:** مراجعة ساكنة موسعة للمصدر والبنية والاختبارات المعلنة. ليست شهادة تشغيل أو اختبار قبول على أجهزة فعلية.

---

## 1. خلاصة القرار

CADCraft ليس مشروع رسم فارغاً يحتاج بناء محرك من الصفر. المستودع يحتوي محرك CAD أصلياً بلغة Rust، وسجل أوامر، ونظام تفاعلي، ومكتبة تنسيقات، وواجهة egui قابلة لإعادة الاستخدام. الطريق الصحيح هو إبقاء المحرك الأصلي وتطوير مضيف Android وخدمات المنصة وتجربة لمس مستقلة.

لكن النسخة الحالية من Android هي **مضيف أولي قابل للبناء** وليست بعد تطبيق CAD هاتفياً مكتملاً. أهم العوائق المثبتة:
1. Android ينشئ `CadApp` مع `Services::default()`؛ منتقي الملفات وSAF غير موصولين.
2. واجهة خدمات الملفات مبنية حول اختيار مسار نصي، بينما SAF يعيد URI/streams، ويلزم جسر صحيح لا تحويل URI إلى مسار مزيف.
3. تخطيط اللوحات ما زال Dock مكتبيّاً، حتى لو ضُبطت عروض أعمدته على Android.
4. عناصر القائمة والأوامر الكثيرة لا تعني أن كل مسار تفاعلي ملائم للمس أو أن dialogs صالحة للشاشة الصغيرة.
5. CI الحالي يبني ARM64 APK ويتحقق من بنية الحزمة، ولا يشغّل اختبارات Rust أو Android instrumentation أو اختبار جهاز.
6. README نفسه يصرح أن SAF، واختبار دورة الحياة، والتحقق من GPU، وتجربة الهاتف/التابلت ما تزال معلقة.

**التوصية:** عدم استبدال المحرك، وعدم البدء بتجميل الأزرار. أولاً نكمل جرد الأوامر واعتمادياتها، ثم نبني طبقة خدمات Android، ثم نظام التفاعل والواجهة الهاتفية، وبعدها نغلق الصيغ والتعريب والأداء باختبارات قابلة للتكرار.

## 2. أدلة المصدر التي تمت مراجعتها

| الملف | ما يثبته |
|---|---|
| `crates/engine/src/cmd.rs` | سجل مركزي `command_specs()` يجمع مواصفات الأوامر من وحدات المجال؛ كل مواصفة تملك id/label/menu/aliases/params/enabled/run/undoable/interactive |
| `apps/cadcraft-android/src/lib.rs` | مضيف Android يستدعي `CadApp::new(Session::new(), Services::default())` ويثبت codecs |
| `crates/ui-egui/src/lib.rs` | `Services` يوفر pick_open/pick_save/pick_lisp/pick_pgp وstart_pick؛ `pick_then` يعتمد على هذه الخدمات |
| `apps/cadcraft-android/README.md` | يقر صراحة بأن SAF غير موصول، وDWG worker غير مفعل/غير مضمون، والتحقق على الجهاز لم يتم |
| `apps/cadcraft-android/app/src/main/java/.../MainActivity.java` | النشاط الحالي يرث GameActivity فقط ويحمل مكتبة main؛ لا يوجد جسر Java/Kotlin لمنتقي المستندات |
| `apps/cadcraft-android/app/src/main/AndroidManifest.xml` | Activity أساسية وRTL flag؛ لا توجد Intent filters لفتح ملفات CAD أو استقبال مستندات |
| `crates/ui-egui/src/cmdline.rs` | سجل الأوامر، Recent Input، IME/preedit، Tab completion وDynamic Input |
| `crates/ui-egui/src/opening.rs` | فتح الخلفية الحالي يستقبل `Path` ويستدعي `read_file(&Path)` |
| `crates/ui-egui/src/exchange.rs` | IMPORT/EXPORT/PUBLISH يطلبون Pick Open/Save؛ المعاينة تنشئ PNG؛ Print يعود إلى PLOT |
| `crates/ui-egui/src/docking.rs` | Canvas وToolSets وLayers وProperties وExternalRefs وBlocks، مع أعمدة Android ثابتة العرض |
| `crates/ui-egui/src/menus.rs` | قائمة مرجعية واسعة وأوامر UI إضافية؛ الواجهة مبنية كقائمة سطح مكتب |
| `crates/ui-egui/src/clipboard.rs` | مزامنة clipboard النظام عبر أحداث egui، وتتطلب تحققاً فعلياً على Android |
| `crates/io/Cargo.toml` و`crates/io/src` | اعتماديات DXF وDWG وSVG والصور وPDF وطبقة plot |
| `.github/workflows/android-build.yml` | build ARM64 وassembleDebug وZIP/package validation فقط |

## 3. جرد القدرات الوظيفية في المحرك

القائمة أدناه جرد مجموعات من سجل الأوامر ومجلداته ومصادرها، وليست ادعاء بأن كل أمر داخل المجموعة تم تشغيله أو اختباره على Android.

| المجموعة | أمثلة مثبتة من سجل الأوامر/المصدر | واجهة الاستخدام الموجودة | المخاطر/العمل المطلوب للهاتف |
|---|---|---|---|
| إنشاء هندسة 2D | LINE, PLINE, CIRCLE, ARC, RECTANG, POLYGON, ELLIPSE, POINT, XLINE, RAY, SPLINE, DONUT, SOLID | Canvas + interactive commands | شريط أدوات لمس، اختيار نقاط دقيق، snapping، إلغاء/إكمال واضح |
| إنشاء هندسة متقدمة | 3-point/center/tangent arcs & circles, ellipse arc, DIVIDE, MEASURE, REVCLOUD, WIPEOUT, 3DPOLY, HELIX, CENTERLINE | command prompts وDynamic Input | تحويل كل prompt إلى touch-first controls واختبار مراحل الأمر |
| التعديل والتحويل | ERASE, MOVE, COPY, ROTATE, SCALE, MIRROR, STRETCH, OFFSET, TRIM, EXTEND, FILLET, CHAMFER, EXPLODE, BREAK, JOIN | interactive factories وgrips | تأكيد selection/target، منع تضارب اللمس مع pan/pinch، hit testing |
| تعديل المنحنيات | PEDIT, SPLINEDIT, LENGTHEN, ALIGN, BLEND, REVERSE, NCOPY, MOCORO, CHSPACE, FLATTEN, ARRAYPATH | prompts وgrip edit | واجهات خيارات صغيرة، تحكم في الرؤوس والمقابض |
| التحديد | SELECT, SELECTALL, QSELECT, SELECTSIMILAR, ISOLATEOBJECTS, HIDEOBJECTS | canvas pick/window selection | tap/long press، نافذة crossing/window، cycling، تحديد متعدد |
| Grips | Grip Move/Stretch/Rotate/Scale/Mirror/Grip Edit | canvas grips وgrip_dyn | تكبير مساحة اللمس، منع gesture ambiguity، اختبارات finger/stylus |
| الطبقات | LAYER, NEW, SET, CURRENT, DELETE, ISOLATE, FREEZE, OFF, LOCK, ON, THAW, PREVIOUS, LAYERSTATE, MERGE | Layers palette + dialogs | لوحة هاتف مخصصة، بحث/فرز، أزرار حالة، bulk operations |
| الخصائص | PROPERTIES, CHPROP, CHANGE, MATCHPROP, COLOR, LINETYPE, LINEWEIGHT, UNITS, TEXTSTYLE, DIMSTYLE, TABLESTYLE | Properties palette ومدراء | محرر خصائص متجاوب وحقول رقمية مع IME |
| الأبعاد والتعليقات | DIMLINEAR, DIMALIGNED, DIMRADIUS, DIMDIAMETER, DIMANGULAR, DIMARC, DIMORDINATE, QDIM, MLEADER, LEADER, DIMSTYLE | commands + style managers | prompts متعددة الخطوات، تحرير النص/الموقع باللمس |
| التهشير والمناطق | HATCH, GRADIENT, BOUNDARY, REGION, HATCHEDIT, HATCHPICK | hatch dialog وpick | اختبار التقاط الحدود، كثافة النمط، preview على الشاشة الصغيرة |
| الكتل والمراجع | BLOCK, INSERT, MINsert, ATTDEF, PURGE, dynamic block data/grips, XREF modules, XCLIP, WBLOCK | Blocks palette وExternal References | SAF للمراجع الخارجية، إدارة مسارات، إدراج ملفات عبر URI |
| الصور | IMAGEATTACH, IMAGE | canvas image rendering | SAF read URI، تدوير/قص/تحجيم، استعادة صلاحية الوصول |
| الجداول | TABLE, SET CELL, INSERT/DELETE ROW/COLUMN, MERGE/UNMERGE | table commands | محرر خلايا قابل للمس وscroll متداخل |
| التخطيطات والطباعة | LAYOUT, MVIEW, VPCLIP, VPORTS, PAGESETUP, PLOT, EXPORTPDF, PLOTWINDOW | dialogs وPreview | تحكم في الورق والمقياس والهوامش وPDF/SAF |
| ملفات وتبادل | NEW, OPEN, CLOSE, QSAVE, SAVEAS, SCRIPT, IMPORT, EXPORT, PREVIEW, PUBLISH, PDFBOOK, ATTACH, SHARE | file menu + exchange picker | SAF، استيراد/تصدير/مرفقات/سكريبتات وعمليات مشاركة |
| القياس والاستعلام | DIST, ID, LIST, AREA, TIME, STATUS, INSPECT, ENTITIES, SHAPES, LOOPS, COUNT | command output/history | عرض نتائج منظم وقابل للنسخ دون تغطية canvas |
| إعدادات الرسم | ORTHO, GRID, SNAP, POLAR, OTRACK, DYNMODE, OSNAP, DSETTINGS, SYSVAR | status bar/settings | شريط حالة هاتف + تبديل سريع + مؤشرات مرئية |
| العرض والتنقل | ZOOM, PAN, REGEN, VIEW, UCS, VIEWPORTS | viewport/canvas | فصل gesture navigation عن drawing gestures |
| القيود والمعلمات | GEOMCONSTRAINT, AUTOCONSTRAIN, DIMCONSTRAINT, PARAMETERS, CONSTRAINTBAR | managers/commands | تحديد/تحرير دقيق، لوحات قابلة للطي |
| النصوص وLISP | TEXT, MTEXT, TXT2MTXT, APPLOAD, LISP, SCRIPT | text dialog/command line | Android document access، IME عربي/إنجليزي، sandbox/أمان |
| المجموعات وترتيب العرض | GROUP, UNGROUP, GROUPEDIT, DRAWORDER, HATCHTOBACK | command/UI | اختيار مجموعات وتغيير stacking |
| أدوات عامة | QUICKCALC, FIND, MEASUREGEOM, RADIUS, ANGLE, VOLUME, AUDIT, RECOVER | dialogs/command output | نتائج واضحة واختبارات عدم فقد البيانات |

### كيف نغلق جرد الأوامر بالكامل

يجب استخراج `command_specs()` من build/test harness إلى JSON/CSV تلقائي يتضمن لكل CommandSpec:
- id وlabel وmenu وaliases وparams.
- `interactive.is_some()`، وfactory/handler.
- enabled reason ومتطلبات document/selection.
- وجود test مباشر أو اختبار تكاملي.
- كل استخدامات `Pick` أو file hooks أو filesystem أو clipboard أو native window.
- تجربة Android: ready / partial / blocked / not assessed مع دليل اختبار.
- owner/priority/acceptance test.

لا يكفي regex على أسماء الأوامر؛ يلزم تتبع handler إلى Interactive state machine، والـ prompt والـ finish/cancel والـ UI caller.

## 4. تصنيف فجوات Android

### P0 — التخزين والملفات (مانع لإصدار عملي)
- خدمات pick_open/pick_save/pick_lisp/pick_pgp/start_pick غير مهيأة في Android host.
- SAF لا يعيد مساراً عادياً يمكن تمريره بأمان إلى Path.
- `opening.rs` والعديد من مسارات engine تستخدم Path/PathBuf أو عمليات filesystem.
- المراجع الخارجية/الصور/الخطوط/السكريبتات/القوالب تحتاج سياسة URI وpersistable permissions.
- لا يوجد intent handling موثق لفتح ملف CAD من مدير الملفات.

### P1 — اللمس والتفاعل (مانع للاستخدام الاحترافي)
- تثبيت تحويل Touch Start/Move/End/Cancel إلى press state sequence ومنع تداخل pinch مع رسم.
- ضمان commit واحد عند release، وعدم اختفاء الشكل المؤقت.
- gesture arbitration: tap/draw, long-press/context, two-finger pan/pinch, stylus.
- تكبير grip targets، snap targets، selection handles.
- عدم اعتماد إحداثيات ثابتة غير متناسبة مع density أو مساحة الإصبع.

### P2 — واجهة الهاتف والتابلت
- تخطيط الهاتف: Canvas كامل تقريباً، Top command/action bar، bottom tool dock، command console قابلة للسحب، panels كـ bottom sheets.
- تخطيط التابلت: dock متعدد الأعمدة، toolsets والطبقات والخصائص مع مساحة canvas مريحة.
- فصل breakpoints عن ثوابت أعمدة 232px الحالية؛ استعمال available width وorientation وdensity.
- إخفاء/إظهار اللوحات دون فقد selection أو الأمر الجاري.
- dialogs وmenus وtooltips وViewCube/status bar تحتاج مراجعة overflow وkeyboard insets.
- حفظ profile/layout مستقل للهاتف والتابلت.

### P3 — Command Console وIME
- console سفلية قابلة للتوسيع مع history وrecent input.
- معالجة IME insets وresize/pan للوحة الرسم.
- Enter/Tab/Escape/Backspace/arrow keys وcomposition/preedit.
- إدخال عربي وإنجليزي مختلط وأرقام عشرية وإحداثيات وsymbols.
- autocomplete قابل للمس مع عدم سرقة focus.

### P4 — الصيغ والملفات التابعة
- جرد codecs والـ features وcfg لكل format.
- DXF read/write round-trip مع preserve unknown/proxy data.
- SVG/PNG/PDF import/export حسب المعلن.
- DWG worker غير مدعوم/غير مثبت في Android بحسب README؛ لا نعد به حتى ينجح build/read/write ARM64.
- اختبار XREF وimage/font/linetype/hatch pattern paths عبر SAF.
- اختبار ملفات كبيرة وحدود capped read والذاكرة.

### P5 — العربية وRTL
- لا يكفي وجود ar.tsv؛ يجب فحص missing keys/fallback.
- اختبار bidi للنصوص التي تجمع أسماء أوامر إنجليزية وإحداثيات وأرقام عربية/لاتينية.
- تشكيل الخط العربي داخل canvas والنصوص والأبعاد وPDF/SVG.
- اتجاه الأيقونات والقوائم والتنقل، وعدم عكس محاور الرسم.
- accessibility content descriptions بالعربية.

### P6 — دورة الحياة والأداء
- pause/resume، rotation، process death، low-memory، permission revocation.
- استعادة GPU/wgpu surface/context.
- autosave/recovery وإدارة عدة مستندات.
- قياس FPS/frame time وRAM والحرارة على ملفات صغيرة ومتوسطة وكبيرة.
- تجنب تجميد UI عند decode/export والـ xref.

## 5. الخطة التنفيذية المقترحة

### المرحلة 0 — إغلاق جرد المصدر (M0)
**المخرجات**
1. مولد inventory آلي من command_specs.
2. ربط UI menu/toolset لكل أمر.
3. تصنيف filesystem/native dependency لكل handler.
4. سجل الاختبارات الموجودة والمفقودة.
5. feature matrix لكل التنسيقات.

**معايير القبول**
- كل CommandSpec مسجل مرة واحدة؛ لا IDs مكررة.
- كل أمر مصنف إلى interactive أو direct-run.
- لكل أمر مصدر implementation واختبار/سبب غياب اختبار.
- لا يبقى unknown في أي أمر من الفئات الأساسية.

### المرحلة 1 — طبقة Android Platform Services (M1)
**التنفيذ**
1. إضافة Android bridge في Java/Kotlin لفتح/إنشاء مستند عبر ACTION_OPEN_DOCUMENT وACTION_CREATE_DOCUMENT وACTION_OPEN_DOCUMENT_TREE عند الحاجة.
2. تمرير URI وdisplay name وMIME type وpersistable read/write grants.
3. إضافة abstraction داخل Rust لـ DocumentRef/ReadSource/WriteTarget أو ما يعادلها، يدعم Path desktop وURI stream Android.
4. تغيير read/write/import/export hooks بحيث تستقبل bytes/reader/writer بدلاً من افتراض Path حيثما يلزم.
5. ربط Pick::Open/Save/Lisp/Pgp مع callbacks غير متزامنة وآمنة.
6. حفظ مستند Android عبر كتابة مؤقتة/محتوى كامل ثم flush/close، ومعالجة فشل/إلغاء العملية.
7. التعامل مع xrefs والصور والخطوط والسكريبتات والمرفقات مع persistable URI grants.
8. إضافة intents لفتح ملفات DXF/DWG المدعومة من مدير الملفات.

**الاختبارات**
- اختيار/إلغاء/حفظ/إعادة فتح.
- مزود Downloads وGoogle Drive إن كان متاحاً ومزود محلي.
- revoke permission وفشل مساحة التخزين.
- حفظ فوق مستند موجود وحفظ باسم جديد.
- مستند كبير مع عدم تجميد الواجهة.

### المرحلة 2 — تثبيت نموذج اللمس (M2)
**التنفيذ**
1. توحيد Touch/Pointer في input state machine واحدة.
2. multi-touch يدخل navigation mode ويلغي press المؤقت دون commit.
3. release ينهي الأمر مرة واحدة، وcancel لا ينشئ هندسة.
4. دعم long press وtap selection وstylus pressure/hover إن دعمته المكتبة.
5. ضبط hit targets حسب density وminimum touch size.
6. وضع اختبار هندسي يحسب entity count/geometry بعد كل gesture.

**اختبارات القبول**
- LINE: down → move → up ينشئ خطاً ثابتاً.
- سحب أثناء الحركة ثم pinch لا ينشئ خطاً شبحياً.
- pan/pinch لا يغير geometry.
- اختيار وتحريك وgrip edit وsnap.
- اختبار سريع متكرر 100 دورة gesture دون تعليق/تكرار.

### المرحلة 3 — تصميم واجهة هاتف أصلية (M3)
**التنفيذ**
1. إضافة MobileLayout مستقل فوق CadApp دون fork لمحرك CAD.
2. portrait/landscape breakpoints ديناميكية.
3. top bar للملف/undo/redo/search/settings.
4. bottom tool dock للفئات الأكثر استخداماً.
5. Command Console bottom sheet.
6. Layers/Properties/Blocks/ExternalRefs كـ sheets على الهاتف.
7. menus وdialogs بملء مناسب، ومساحة canvas قابلة للاسترجاع.
8. حفظ حالة panel/command والاختيار عند فتح sheet.

**اختبارات القبول**
- شاشة 5.5–6.7 بوصة بدقة وكثافات مختلفة.
- لا تداخل بين command console وkeyboard وcanvas.
- لا قص للأزرار أو القوائم في portrait/landscape.
- لا يقل canvas عن مساحة عملية أثناء الرسم.

### المرحلة 4 — تجربة التابلت (M4)
**التنفيذ**
1. dock مرن مبني على available width لا أرقام ثابتة.
2. toolsets يسار، canvas وسط، layers/properties يمين عند المساحة الكافية.
3. floating panels والـ split resizing.
4. دعم keyboard/mouse/stylus مع touch.
5. layout profile منفصل للتابلت.

**اختبارات القبول**
- 8–13 بوصة، portrait وlandscape.
- split-screen / multi-window إن دعمته المنصة.
- dock لا يحجب canvas ولا يخرج خارج الشاشة.

### المرحلة 5 — Command Console والكتابة (M5)
**التنفيذ**
1. إعادة ترتيب CmdLine للهاتف دون تغيير command engine.
2. لوحة نتائج قابلة للنسخ والبحث.
3. autocomplete list كبيرة بما يكفي للمس.
4. IME integration واختبار composition.
5. دعم لوحة مفاتيح مادية واختصارات.

### المرحلة 6 — توصيل الأوامر والأدوات (M6)
**التنفيذ حسب الأولوية**
- Tier A: LINE/PLINE/CIRCLE/ARC/RECTANG، SELECT، MOVE/COPY/ROTATE/SCALE، ERASE، OFFSET، TRIM/EXTEND، UNDO/REDO، ZOOM/PAN، LAYER، DIST/AREA، TEXT/MTEXT، DIM، HATCH، SAVE/OPEN/EXPORT.
- Tier B: blocks/attributes، arrays، groups، xrefs، images، tables، layouts/viewports، constraints، advanced curves.
- Tier C: scripts/LISP، compare, batch publish, recovery and advanced managers.

**شرط التنفيذ**
لا نعيد كتابة كل أمر. نضيف adapter/interaction UI للأمر القائم حيث أمكن، ثم نختبره end-to-end من زر الهاتف ومن command line.

### المرحلة 7 — صيغ الملفات والطباعة (M7)
- مصفوفة format × operation × Android ABI.
- DXF open/save/round-trip ونسخ متعددة من ملفات مرجعية.
- SVG/PNG/PDF import/export مع فتح الملف الناتج في تطبيق آخر.
- PLOT/preview/paper orientation/scale/lineweights.
- DWG يبقى experimental/unsupported حتى اختبار bridge/worker ARM64 واقعي.
- فحص preserve of unsupported entities/proxy records.

### المرحلة 8 — RTL والعربية (M8)
- missing translation report.
- مراجعة اتجاهات الواجهات وليس قلب الرسم.
- نص عربي في canvas وMTEXT والأبعاد والجداول.
- PDF/SVG export fonts/shaping.
- اختبار النص المختلط وIME والنسخ واللصق.

### المرحلة 9 — الاستقرار والأداء (M9)
- lifecycle and GPU recovery.
- autosave/recovery.
- قياس زمن فتح/حفظ/رسم/تصدير.
- memory budgets وlarge drawing stress.
- crash/ANR monitoring وlogs قابلة للتصدير.

### المرحلة 10 — Release Candidate (M10)
- APK arm64 debug ثم signed release.
- smoke tests آلية + instrumentation.
- اختبار على جهاز هاتف وجهاز تابلت فعليين.
- تقرير نتائج مع APK SHA256 وcommit SHA ومصفوفة النجاح/الفشل.
- لا إصدار نهائي قبل إغلاق P0/P1 وجميع اختبارات القبول الحرجة.

## 6. خطة CI والاختبارات

### CI المقترح
1. `cargo fmt --all -- --check`.
2. `cargo test -p cadcraft-engine`.
3. `cargo test -p cadcraft-io`.
4. `cargo test -p cadcraft-ui-egui` مع إعدادات headless المناسبة.
5. parity checks الموجودة في xtask.
6. `cargo ndk ... build --release -p cadcraft-android`.
7. Gradle assembleDebug.
8. APK inspection: ABI, native lib, manifest, package integrity.
9. Android emulator smoke/instrumentation tests عندما يمكن تشغيل بيئة emulator.
10. Artifact metadata: commit, SHA256, ABI, versionCode/versionName.

### سجل اختبارات قبول الجهاز

| ID | الاختبار | شرط النجاح | الحالة عند هذا التدقيق |
|---|---|---|---|
| D-001 | install/launch ARM64 | فتح دون crash | لم يثبت على جهاز |
| D-002 | pause/resume/rotation | لا فقد للمستند أو surface | لم ينفذ |
| F-001 | SAF Open DXF | فتح الرسم واسم الملف صحيح | محجوب: bridge غير موصول |
| F-002 | SAF Save/Save As | حفظ وإعادة فتح بلا فقد | محجوب: bridge غير موصول |
| F-003 | Import/Export/PDF | ناتج صحيح قابل للفتح خارجياً | لم ينفذ |
| T-001 | LINE release | كيان ثابت واحد بعد رفع الإصبع | لم يثبت على build الحالي |
| T-002 | pinch/pan while drawing | لا geometry شبحية | لم ينفذ |
| T-003 | select/grips/snap | اختيار وتعديل موثوق | لم ينفذ |
| C-001 | command + autocomplete | تنفيذ الأوامر دون فقد IME | لم ينفذ |
| U-001 | phone portrait/landscape | لا clipping/overlap | لم ينفذ |
| U-002 | tablet docks | panels/canvas تعمل | لم ينفذ |
| L-001 | Arabic mixed text | اتجاه وتشكيل صحيحان | لم ينفذ |
| P-001 | large drawing stress | لا ANR/نفاد ذاكرة | لم ينفذ |
| X-001 | format round trip | لا فقد غير مبرر للبيانات | اختبارات المكتبة ليست إثباتاً على Android |

كل نتيجة فعلية يجب أن تسجل الجهاز، Android API، density، commit، APK SHA256، الخطوات، logcat، النتيجة.

## 7. ترتيب التنفيذ الذي أوصي به

| الأولوية | حزمة العمل | سبب الترتيب | Definition of Done |
|---|---|---|---|
| P0 | M0 inventory generator | يمنع إغفال أدوات أو ادعاء دعم غير مثبت | سجل آلي لكل command ID والاختبارات والاعتماديات |
| P0 | SAF + URI IO | بدونها لا يمكن استخدام الرسومات والملفات بشكل طبيعي | فتح/حفظ/تصدير/استيراد على جهاز |
| P0 | touch state machine | الرسم باللمس هو الوظيفة الأساسية | لا اختفاء/تكرار/تداخل gestures |
| P1 | mobile phone layout | قابلية استخدام حقيقية | جميع الشاشات الأساسية بلا قص |
| P1 | command console + IME | أوامر CAD تحتاج إدخالاً موثوقاً | اختبارات العربية والإنجليزية والاختصارات |
| P1 | tablet responsive docks | الاستفادة من مساحة التابلت | dock قابل للتحجيم والطي |
| P1 | file formats + references | توافق الرسومات الفعلية | round-trip وexternal references |
| P2 | Arabic/RTL polish | تجربة عربية متسقة | تقرير ترجمة ولقطات قبول |
| P2 | performance/lifecycle | موثوقية الملفات الكبيرة والجلسات الطويلة | stress + lifecycle suite |
| P2 | advanced command tiers | إغلاق الفجوات المتقدمة | كل أمر مصنف ومختبر أو معلن محدوداً |

## 8. قواعد تمنع إعلان نجاح غير حقيقي

- نجاح build ليس نجاح تثبيت أو تشغيل.
- وجود CommandSpec ليس نجاح تفاعل لمس.
- وجود codec ليس نجاح SAF.
- وجود ar.tsv ليس تعريباً كاملاً.
- نجاح اختبار desktop لا يثبت Android.
- لا نعلن DWG على Android دون build/read/write اختبارياً.
- لا نعلن أن كل الأدوات تعمل إلا بعد inventory واختبار لكل command ID أو توثيق الاستثناء.
- لا ننقل/نستبدل المحرك الأصلي إلا عند وجود عيب مثبت لا يمكن إصلاحه بطبقة host/adapters.

## 9. الملفات التالية التي يجب أن تنتج عن التنفيذ

1. `docs/android-command-inventory.json` مولد من command_specs.
2. `docs/android-command-coverage.md` خريطة أمر → UI entry → engine handler → test.
3. `docs/android-platform-services.md` مواصفات SAF/URI/clipboard/permissions.
4. `docs/android-device-test-results.md` سجل الاختبارات الفعلية.
5. `apps/cadcraft-android` Android bridge وhost services.
6. `crates/ui-egui` MobileLayout وtouch adapters وconsole presentation.
7. `.github/workflows/android-build.yml` CI موسع للاختبارات والحزمة.

**النتيجة النهائية لهذا التدقيق:** تمت مراجعة بنية المحرك ومجموعات الأوامر ومصادر Android والملفات واللوحات وCI مراجعة أوسع، وتم تحديد خطة تنفيذ مرتبة. ما زال الإغلاق النهائي لجرد كل CommandSpec، وتتبع كل handler، وتشغيل الاختبارات على Android، عملاً تالياً لا يجوز الادعاء بأنه اكتمل في هذه المرحلة.
