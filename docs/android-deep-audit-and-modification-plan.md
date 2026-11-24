# CADCraft Android — التدقيق المعمّق وخطة التعديل التنفيذية

**المستودع:** `loay94an/CAD`  
**الفرع:** `feat/android-phone-tablet`  
**المرجع المفحوص:** `87b2070a04377462117d5e273c1c84b646ebff75`  
**التاريخ:** 2026-11-24  
**نوع التدقيق:** قراءة ساكنة موسعة لمصدر المحرك والواجهة ومضيف Android؛ لم تُنفذ بعد اختبارات قبول على جهاز Android.

## 1. حدود التدقيق بدقة

تمت مراجعة شجرة Git كاملة دون truncation (1,415 مساراً في المرجع الحالي)، وقراءة ملفات التسجيل الأساسية للأوامر، و40 ملفاً رئيسياً من وحدات أوامر المحرك، ومصادر Android/egui ذات الصلة بالملفات والقوائم واللوحات واللمس.

**نتيجة العد:** عُثر على 387 تصريحاً صريحاً من نمط `CommandSpec::new("...")` في ملفات وحدات الأوامر الأربعين التي فُحصت. هذا عدد تصريحات صريحة في ملفات المصدر، وليس ادعاءً بأنه العدد النهائي لكل أمر متاح في runtime؛ فهو لا يحسب تلقائياً:
- الأوامر/الاختصارات التي تنشئها `file::alias_specs()` أو أي builders ديناميكية.
- أوامر واجهة المستخدم `UI_COMMANDS` التي لا تمر عبر محرك الأوامر.
- أوامر/أسماء LISP والاختصارات التي تُحمّل وقت التشغيل.
- أي factory/helper قد ينشئ مواصفات دون نص `CommandSpec::new` مباشر.

لم تُتبع جميع دوال التنفيذ داخل كل ملف إلى كل helper أو كل اختبار؛ لذلك تبقى هذه مراجعة سجلّ/عائلات أوامر، وليست شهادة أن كل أمر اختُبر سلوكياً.

## 2. سجل عائلات الأوامر

الأرقام أدناه عدد تصريحات `CommandSpec::new` الظاهرة في الملف المحدد فقط، وليست درجات اكتمال Android.

| مجال الوظيفة | ملف التسجيل | التصريحات |
|---|---|---:|
| ملفات، فتح، حفظ، script | `cmd/file.rs` | 9 |
| تدقيق واسترداد | `cmd/audit.rs` | 3 |
| تراجع/إعادة/قص/نسخ/لصق/تحديد | `cmd/edit.rs` | 15 |
| تحديد متقدم | `cmd/qselect.rs` | 2 |
| تحديد مشابه/إخفاء/عزل | `cmd/selecttools.rs` | 4 |
| Zoom/Pan/View/Redraw | `cmd/view.rs` | 22 |
| UCS | `cmd/ucs.rs` | 7 |
| رسم أساسي | `cmd/draw.rs` | 16 |
| رسم متقدم/أقواس/منحنيات/3D | `cmd/draw2.rs` | 28 |
| أبعاد وتعليقات وقيادات | `cmd/annotate.rs` | 24 |
| أنماط الأبعاد | `cmd/dimstyles.rs` | 2 |
| CLI لأنماط الأبعاد | `cmd/dimstyle_cli.rs` | 2 |
| تحرير MLeader | `cmd/mleader_edit.rs` | 5 |
| Annotation scale | `cmd/annoscale.rs` | 9 |
| Hatch/Gradient/Boundary/Region | `cmd/hatch.rs` | 7 |
| إزالة التكرارات | `cmd/overkill.rs` | 1 |
| Blocks/Insert/Attributes/Purge | `cmd/blocks.rs` | 14 |
| تعديل هندسي أساسي | `cmd/modify.rs` | 17 |
| ترتيب الرسم | `cmd/draworder.rs` | 11 |
| Arrays | `cmd/array.rs` | 4 |
| Polyline/Spline/Align/Blend/Lengthen | `cmd/modify2.rs` | 12 |
| Groups | `cmd/groups.rs` | 4 |
| Grip editing | `cmd/gripcmds.rs` | 7 |
| Layers وحالاتها | `cmd/layer.rs` | 24 |
| Layouts/Viewports/Plot | `cmd/layout.rs` | 22 |
| Import/Export/Preview/Publish | `cmd/exchange.rs` | 8 |
| Image attach | `cmd/image.rs` | 2 |
| Properties/Styles/Units/Color | `cmd/props.rs` | 32 |
| Inquiry/Measure/Area/Count | `cmd/inquiry.rs` | 15 |
| Drafting settings/variables | `cmd/settings.rs` | 17 |
| Options | `cmd/options.rs` | 1 |
| Menu/Toolbar | `cmd/interface.rs` | 2 |
| Calculator/Find/Measure geometry | `cmd/utility.rs` | 7 |
| Tables | `cmd/table.rs` | 8 |
| Tolerance | `cmd/tolerance.rs` | 1 |
| TXT to MTEXT | `cmd/txt2mtxt.rs` | 1 |
| Geometric/Dimensional constraints | `cmd/constraints.rs` | 16 |
| APPLOAD/LISP | `cmd/lisp.rs` | 2 |
| Drawing compare | `cmd/file/compare.rs` | 4 |
| Clipboard helpers | `cmd/clipboard.rs` | 0 |

**المجموع الصريح في الملفات أعلاه: 387.** تسجيل الأوامر يجمع هذه العائلات في `crates/engine/src/cmd.rs::command_specs()`، ويخزن لكل أمر id/label/menu/shortcut/aliases/params/enabled/run/undoable/interactive. وهذه نقطة ممتازة لبناء جرد آلي قابل للتحقق بدلاً من تخمين ما تدعمه الواجهة.

## 3. نتائج مضيف Android والواجهات

### 3.1 المضيف والـ lifecycle
- `apps/cadcraft-android/src/lib.rs` ينشئ `eframe::NativeOptions` مع `AndroidApp` ويشغّل `CadApp` و`Session` الأصليين.
- `MainActivity.java` مجرد `GameActivity` ويحمّل `libmain`; لا توجد فيه حالياً معالجة Intent أو Storage Access Framework.
- Manifest يعلن Launcher Activity و`supportsRtl=true`، لكنه لا يعلن حالياً `ACTION_OPEN_DOCUMENT` أو `ACTION_CREATE_DOCUMENT` كمسار استقبال للمستندات.
- Gradle يحدد minSdk 23 وtargetSdk 35 وABI واحداً arm64-v8a. هذا قرار أولي مقبول، لكنه يعني عدم وجود APK x86 أو 32-bit حالياً.

### 3.2 خدمات المنصة والملفات
- `Services` في `crates/ui-egui/src/lib.rs` يقدم `pick_open`, `pick_save`, `start_pick`, `pick_lisp`, `pick_pgp`, و`no_file_system`.
- Android host يمرر `Services::default()`؛ وبالتالي لا توجد callbacks للفتح/الحفظ/APPLOAD/استيراد PGP.
- `pick_then` مصمم فعلاً لمنتقي غير متزامن، ويربط العملية بمستند UID ويحمي من استجابة متأخرة على مستند آخر. يمكن إعادة استخدامه لخدمة SAF بدلاً من اختراع picker جديد في UI.
- `open_path` و`opening::Opening` يعتمدان على `Path/PathBuf` و`read_file(path)` على worker thread. URI من SAF ليس مساراً محلياً مضموناً.
- `install_io` يثبت codecs على bytes (قراءة/كتابة/plot/image/entity/object records)، وهذا أساس جيد لبناء طبقة stream/bytes.
- مسار الحفظ الحالي يتضمن backup/atomic filesystem behavior في المحرك؛ يجب الحفاظ على هذه الضمانات عند الانتقال إلى ContentResolver وعدم استبدالها بحفظ مباشر غير آمن.
- يلزم دعم URI persistence عبر `takePersistableUriPermission` عندما يسمح مزود المستندات بذلك، ومعالجة provider الذي لا يدعم seek/rename/atomic replace.

### 3.3 الواجهة والهاتف
- `UiState::default()` يخفي Tool Sets وPalettes على Android، لكنه يترك Toolbar وFile Tabs وStatus Bar وCommand Line وViewCube مفعّلة.
- `docking.rs` يعرّف Android widths ثابتة مقدارها 232dp لكل من Tool Sets وPalettes. هذه أبعاد لا تتناسب مع هاتف portrait إذا فُتحت اللوحتان معاً؛ الإخفاء الافتراضي يخفف المشكلة لكنه لا يصنع تصميم هاتف متجاوباً.
- `menus.rs` يولد شجرة القوائم من `command_specs()` ثم يضيف `UI_COMMANDS`؛ شريط القوائم على Android قابل للتمرير أفقياً، لكنه ما زال يحتفظ بنموذج القوائم المكتبي.
- `docking.rs` يوفر Canvas/ToolSets/QuickCalc/Layers/Properties/ExternalRefs/Blocks؛ ينبغي أن تتحول لوحات الهاتف إلى sheets/overlays لا أعمدة ثابتة.
- Blocks وXrefs وLayers وProperties ليست مجرد رسومات شكلية: هذه اللوحات تستدعي أوامر المحرك وتحتاج اختبارات تفاعلية مستقلة.
- `cmdline.rs` يضم history/recent input/autocomplete/dynamic input/IME composition، ويجب الحفاظ عليها مع تغيير العرض إلى console سفلي مناسب للمس ولوحة المفاتيح.

### 3.4 اللمس والتنقل
- `canvas/press.rs` يتعامل مع Touch Start/Move/End/Cancel، ويختار stream اللمس الأصلي عندما توجد Touch events لتجنب ازدواج Pointer events.
- `canvas.rs` يستخدم `multi_touch()` ومركز gesture لتحريك/تكبير العرض.
- هذا يثبت وجود إصلاحات لمس في المصدر، لكنه لا يثبت سلوكها على أجهزة متعددة أو ثبات كل الأوامر التفاعلية بعد release.
- يجب اختبار الإيماءات المتزامنة مع نافذة تحديد، snap، grips، dynamic input، command prompts، وفتح قوائم فوق مساحة الرسم.

### 3.5 الصيغ
- `cadcraft-io` يصف DXF وSVG وPNG وPDF ويعتمد أيضاً على `craft-dwg`.
- Android host يثبت read/write/plot وimage decode hooks.
- لا نعلن دعم DWG الكامل أو عدم فقد خصائصه على Android حتى تجتاز ملفات DWG فعلية round-trip واختبارات ARM64؛ README يذكر صراحة أن DWG worker support غير مفعّل/غير معلن على Android.
- يجب اختبار DXF versions، الصور المرتبطة، Xrefs، Fonts، PDF/PNG/SVG، والبيانات/الكيانات غير المعروفة.

## 4. مصفوفة اعتماديات المنصة لكل عائلة

| العائلة | اعتماديات مرجحة يجب تدقيقها في handler | سياسة Android |
|---|---|---|
| OPEN/SAVE/SAVEAS/RECOVER | file picker، path، filesystem، backup، worker | SAF streams/bytes + حفظ آمن |
| IMPORT/EXPORT/PUBLISH/PLOT | picker، output URI، codecs، preview/print | SAF create document + اختبار MIME |
| IMAGE/XREF/INSERT/WBLOCK/BLOCKLIB | قراءة ملفات تابعة ومسارات نسبية | URI resolver + cache داخلي مضبوط + إعادة ربط |
| APPLOAD/LISP/ALIASEDIT | قراءة نصوص LSP/PGP، مجلدات أخيرة | SAF + تأكيد أمان + دعم ملفات نصية |
| COPY/CUT/PASTE | clipboard system + DXF text | ربط Android clipboard والتحقق من الحجم |
| FONTS/TEXT/MTEXT | تحميل خطوط، Unicode shaping، bidi | خطوط Android fallback + اختبار العربية |
| PLOT/PDF | PDF writer، print destination | Android Print Framework أو SAF PDF |
| AUTOSAVE/RECOVERY | filesystem، temp files، atomic rename | مساحة داخلية خاصة + recovery بعد process death |
| OPTIONS/SETTINGS | preferences/storage | DataStore/SharedPreferences أو eframe storage مع migration |
| SPACEMOUSE | HID backend | إخفاء/تعطيل واضح إذا لم يتوفر جهاز |
| SCREEN/CAPTURE/CLIPBOARD | capture، permission، clipboard | Android APIs وruntime permissions حيث يلزم |

هذه المصفوفة قائمة تحقق وليست ادعاء بأن كل اعتماد تم إثباته لكل handler. الخطوة الهندسية التالية هي استخراج call-sites لكل اعتماد وربطها بالـ IDs.

## 5. خطة التعديل التنفيذية — بوابات قبول إلزامية

### M0 — إغلاق الجرد الآلي (P0)
**المخرجات**
1. برنامج/اختبار داخلي يستخرج كل `CommandInfo` من runtime registry أو test harness.
2. ملف CSV/JSON/Markdown لكل أمر: ID، label، aliases، menu path، shortcut، interactive، enabled condition، handler source، test names.
3. ربط `UI_COMMANDS` وpalette routes وmenu entries بأمر أو action.
4. كشف orphan commands (أمر لا يظهر في قائمة ولا toolset) وdead UI actions وmenu item بلا handler.
5. جرد tests لكل module وفصل unit tests عن Android device tests.

**معيار القبول:** لا يوجد ID مسجل دون صف، وكل صف له مصدر handler وحالة اختبار موثقة.

### M1 — طبقة ملفات Android SAF (P0، قبل اعتماد التطبيق)
**التصميم المقترح**
- Java/Kotlin bridge صغير داخل MainActivity يستعمل `ACTION_OPEN_DOCUMENT` و`ACTION_CREATE_DOCUMENT` مع MIME/filter مناسب.
- إرجاع URI وdisplay name وMIME، لا تمرير URI على أنه Path.
- تنفيذ عمليات القراءة والكتابة عبر ContentResolver streams/ParcelFileDescriptor.
- إضافة abstraction محايدة في Rust/واجهة host: `DocumentRef` وعمليات read/write bytes/stream، مع بقاء filesystem adapter لسطح المكتب.
- إعادة استخدام `Services::start_pick` و`pick_then` لتسليم النتيجة asynchronously.
- تخزين URI grants القابلة للاستمرار، وإظهار خطأ واضح إذا سحب المستخدم الصلاحية.
- فتح المستند: read bytes على worker، ثم `cadcraft_io::read(bytes, display_name)`.
- حفظ: `cadcraft_io::write` إلى buffer/temp internal file، ثم كتابة URI مع فحص نجاح flush/close. لا تفترض أن DocumentProvider يدعم rename/atomic replace.
- الملفات التابعة (صور/Xrefs/Fonts/LISP/PGP) تحتاج resolver موحداً وسياسة cache وإعادة ربط؛ لا تحوّل URI إلى filesystem path مزيف.

**اختبارات:** open DXF/DWG، Save، Save As، overwrite confirmation، export PDF/SVG/PNG، cancel، provider error، URI permission persistence، ملف كبير، process death أثناء الكتابة.

### M2 — إدخال اللمس كآلة حالات واحدة (P0)
- توحيد Touch وPointer وتحديد مصدر contact ومنع event duplication.
- حالات: Idle → Pressed → Dragging → Released/Cancelled، مع multi-touch interrupt/cancel صريح.
- لا تثبّت عنصر الرسم قبل release إلا إذا كان الأمر يتطلب stream نقاط (freehand/polyline).
- فصل pan/pinch عن drawing selection باستخدام عدد الأصابع وأولوية gesture.
- منع زر الواجهة أو لوحة الأوامر من تمرير event إلى canvas.
- تحسين hit targets وgrips وselection window وsnap للـ density.
- اختبار قلم stylus وhover/pressure لاحقاً دون جعلها شرط الإصدار الأول.

**اختبارات:** line، polyline، circle، rect، freehand، move، copy، grip stretch، window/crossing selection، snap، cancel، pinch أثناء رسم، فتح menu أثناء prompt.

### M3 — واجهة هاتف حقيقية (P1)
- شاشة رسم كاملة؛ top bar صغير (ملف/تراجع/إعادة/حفظ/لوحات).
- Bottom tool dock قابل للسحب/الطي، أزرار كبيرة مصنفة، وإمكانية تخصيص المفضلة.
- command console سفلي يفتح عند الطلب، لا يستهلك ارتفاع الرسم دائماً؛ عند ظهور IME يرفع نفسه ويقلص canvas دون تغطية الحقل.
- Layers/Properties/Blocks/Xrefs/QuickCalc في bottom sheets أو side sheets بحسب العرض.
- إلغاء الأعمدة الثابتة 232dp من phone layout؛ إبقاء docking المتقدم للتابلت/الشاشات الكبيرة.
- اختصارات واضحة للوصول إلى كل أمر موجود، بما في ذلك الأوامر غير المضافة إلى toolbar.
- لا تخفِ أمرًا مدعوماً بصمت؛ إما يظهر في بحث الأوامر أو يصنف advanced/unsupported بوضوح.

### M4 — واجهة Tablet وDesktop parity (P1)
- عند عرض كافٍ: Canvas وسط، ToolSets يسار، Layers/Properties يمين.
- breakpoints تعتمد العرض الفعلي وwindow size classes لا نوع الجهاز وحده.
- حفظ workspace منفصل للهاتف والتابلت، وتجنب إفساد layouts desktop القديمة.
- دعم landscape/keyboard/mouse/stylus.

### M5 — command console وIME (P1)
- المحافظة على history/autocomplete/aliases/dynamic input/preedit.
- اختبارات Enter/Tab/Escape/arrows/long-press/paste.
- تصحيح اتجاه النص المختلط: command names والملفات والأرقام LTR داخل سياق عربي RTL.
- إدخال الإحداثيات والقيم الرقمية بعلامات عشرية/فواصل متوقعة.
- سجل أوامر قابل للطي مع ارتفاع متغير.

### M6 — ربط الأوامر والأدوات واللوحات (P1)
تقسيم التنفيذ إلى حزم مستقلة:
1. Draw: LINE/PLINE/CIRCLE/ARC/RECT/POLYGON/ELLIPSE/SPLINE/TEXT.
2. Modify/Select: MOVE/COPY/ROTATE/SCALE/MIRROR/TRIM/EXTEND/OFFSET/FILLET/CHAMFER/ERASE/GRIPS.
3. Annotation: dimensions/MLEADER/TEXT styles/annotation scales.
4. Layers/Properties: إنشاء/تبديل/قفل/إخفاء/خصائص.
5. Blocks/Xrefs/Images: attach/insert/edit/reload/path/bind.
6. Layout/Viewport/Plot: model/paper/layout/viewport/plot/preview.
7. Inquiry/Utilities/Constraints/Tables/LISP/Compare.
لكل حزمة: command ID → UI entry → start/interactive prompt → input/cancel → document mutation → undo/redo → persistence/export → test.

### M7 — الصيغ وعدم فقد البيانات (P1)
- DXF fixtures من إصدارات متعددة وملفات كبيرة.
- round-trip entities/styles/layers/blocks/dimensions/hatches/constraints/xrefs/images.
- اختبار DWG read/write على ARM64 بملفات واقعية ومقارنة فقد البيانات؛ تعطيل الخيار أو وسمه تجريبياً إذا لم يجتز.
- PDF/PNG/SVG مع الورق والمقياس والاتجاه والشفافية والخطوط.
- حفظ الملفات التالفة/المجهولة دون حذف صامت للكيانات المدعومة جزئياً.

### M8 — العربية وRTL (P1)
- مقارنة مفاتيح ar.tsv مع اللغة المرجعية واستخراج missing/unchanged/fallback.
- دعم bidi للنصوص التي تحتوي عربية + أوامر CAD لاتينية + أرقام + paths.
- اختبار shaping في الواجهة، النص داخل الرسم، MTEXT، PDF/SVG.
- مراجعة جميع الحوارات ورسائل الخطأ والاختصارات وعناصر الوصول.

### M9 — الأداء والاعتمادية (P1)
- ملفات 1MB/20MB/100MB وعشرات/مئات آلاف الكيانات.
- قياس frame time وinput latency والذاكرة وحرارة الجهاز.
- إيقاف/استئناف، تدوير، فقد GPU context، background/foreground.
- autosave/recovery/process death أثناء فتح/حفظ.
- لا تستخدم worker threads غير محدودة؛ راجع jobs والـ cancellation والذاكرة.

### M10 — جودة الإصدار (P0)
- CI: fmt/clippy/unit tests/Android release build/package validation.
- Emulator smoke test للتثبيت/التشغيل/التدوير ولقطات واجهة.
- جهاز حقيقي واحد على الأقل للهاتف وجهاز Tablet أو emulator كبير.
- test report مرتبط بـ commit وAPK SHA256 وAndroid API/device/density.
- لا تصدر Release على أساس نجاح تجميع APK فقط.

## 6. ترتيب التنفيذ والاعتماديات

| الترتيب | الحزمة | الأولوية | لماذا |
|---:|---|---|---|
| 1 | إغلاق جرد registry وUI mapping | P0 | يمنع نسيان أوامر/وظائف مخفية |
| 2 | SAF + DocumentRef + حفظ آمن | P0 | بدونها لا يوجد سير عمل مستندات مكتمل |
| 3 | touch state machine + تثبيت الرسم | P0 | أساس التفاعل الهندسي |
| 4 | command console + IME | P0 | الوصول للأوامر والمعاملات |
| 5 | phone shell + sheets + responsive | P1 | استعمال عملي على شاشة صغيرة |
| 6 | tablet layout | P1 | الاستفادة من مساحة التابلت |
| 7 | أدوات الرسم/التعديل/اللوحات end-to-end | P1 | التحقق من الوظائف الأساسية |
| 8 | الصيغ/الخطوط/العربية | P1 | موثوقية المستندات والإخراج |
| 9 | lifecycle/performance/recovery | P1 | منع الأعطال وفقد العمل |
| 10 | device QA وإصدار | P0 gate | منع تسليم APK غير مختبر |

## 7. الاختبارات ومعايير النجاح

لكل test case احفظ:
- Test ID، commit SHA، APK SHA256.
- الجهاز، ABI، Android API، screen width/height/density.
- خطوات إعادة الإنتاج، النتيجة، logcat، screenshot/recording.
- هل الاختبار آلي أم يدوي، وهل يعاد عند تغيير المحرك/الواجهة.

**بوابات الإصدار الإلزامية**
1. 100% من IDs المسجلة موثقة في inventory.
2. لا يوجد أمر أساسي (Draw/Modify/Save/Select/Layer) بلا مسار UI أو command search.
3. لا يوجد فقد صامت عند فتح/حفظ/تصدير fixtures.
4. نجاح فتح وحفظ وإعادة فتح ملف من SAF.
5. ثبات العناصر بعد release، وعدم تداخل pinch/pan مع الرسم.
6. كل القوائم واللوحات قابلة للوصول ولا تتجاوز حدود الهاتف.
7. Arabic/English mixed input works in command console and drawing text.
8. لا crash في pause/resume/rotation/recovery smoke tests.
9. توثيق أي أمر غير مدعوم أو جزئي للمستخدم بدلاً من إظهاره كأنه يعمل.
10. نجاح CI والاختبار الفعلي على جهاز، وليس package validation فقط.

## 8. ما لم يُنفذ بعد (بشفافية)

- لم يتم تشغيل Rust workspace tests في هذه الجولة.
- لم يتم تنفيذ Android emulator/device tests.
- لم يتم تنفيذ SAF bridge أو أي تعديل برمجي وظيفي؛ هذه الوثيقة خطة تدقيق وتعديل.
- لم تتم مراجعة جميع handlers والـ helper calls لكل تصريح من الـ 387 تصريحاً.
- لم يتم قياس الأداء أو الذاكرة أو توافق ملفات فعلية.
- لا يوجد في هذا التدقيق ما يبرر وصف التطبيق بأنه مكتمل أو جاهز للإصدار.

**القرار:** ابدأ M0 inventory الآلي ثم M1 SAF وM2 touch؛ لا تبدأ إعادة تصميم عشوائية ولا تغيّر المحرك الأصلي. احتفظ بمحرك CADCraft كمصدر الحقيقة، واجعل Android host مسؤولاً عن خدمات النظام وتجربة اللمس والتخطيط.


---

## 9. سجل التنفيذ الفعلي — 2026-11-24

### التعديل الأول: إلغاء اللمسة لا يساوي رفع الإصبع
- الملفات: `crates/ui-egui/src/canvas/press.rs` و`crates/ui-egui/src/canvas.rs`.
- قبل الإصلاح: `TouchPhase::Cancel` كان يتحول إلى `Step::Up` مثل `TouchPhase::End`.
- بعد الإصلاح: تمت إضافة `Step::Cancel` مستقل، ويمسح الضغط الجاري دون تشغيل مسار release/grip/window completion.
- اختبار regression جديد: `native_touch_cancel_is_not_a_release`.
- Commits:
  - `98456757e12f5ba33f46ed32a43fba590c7e4d90`
  - `dda658491d661c54cd4a1a569f367345ce25d0c5`

### تحسين التحقق الآلي
- أضيفت خطوة إلى `.github/workflows/android-build.yml` لتشغيل:
  `cargo test -p cadcraft-ui-egui --lib canvas::press::tests`
  قبل بناء APK.
- Commit: `b2a26f42402ea572fb56a4ed0cfc61ca89d6159e`.
- Workflow run: https://github.com/loay94an/CAD/actions/runs/45306771988
- عند تحديث هذا السجل كان التشغيل قيد التنفيذ؛ لا تسجل نجاح الاختبار أو APK قبل ظهور نتيجة مكتملة.

### ما بقي في M2
- اختبار سلوك الإلغاء بعد بداية أمر تفاعلي؛ لأن نمط المحرك يطبق بعض النقرات على Down، وليس فقط على Up.
- اختبار TouchId عند وجود أكثر من إصبع، والانتقال بين single-touch وmulti-touch.
- اختبار end-to-end على Android فعلي، وليس اختبار تحويل الأحداث وحده.

### M1 — SAF
- لم يُنفذ جسر SAF بعد.
- ثبت أن Android يمرر `Services::default()`، وأن `Services::start_pick` هو مسار اختيار الملفات غير المتزامن القابل لإعادة الاستخدام.
- لا يجوز تمرير Content URI إلى `Path`؛ لأن `opening::Opening` و`engine::cmd::file` يقرآن المسارات عبر filesystem، كما أن الحفظ والنسخ الاحتياطي يعتمدان عمليات ملفات ذرية.
- يلزم تصميم Android document adapter يغطي القراءة والكتابة والملفات التابعة (Xrefs/images/fonts/scripts)، لا مجرد إضافة Intent إلى Java.

### حالة التنفيذ بعد أول إصلاح
- [x] تمييز Touch Cancel عن Touch End في event mapping.
- [x] منع Cancel من إنهاء pending press كأنه release.
- [x] إضافة اختبار regression لوصول Cancel.
- [x] إضافة اختبار اللمس إلى Android CI.
- [ ] انتظار نتيجة CI على commit الحالي.
- [ ] بناء SAF document adapter.
- [ ] استكمال اختبارات multi-touch والرسم.
- [ ] تنفيذ واجهة الهاتف والتابلت.
- [ ] اختبار جهاز فعلي.

**ملاحظة:** هذا سجل تقدم مرحلي، ولا يعني اكتمال التحويل أو صلاحية APK للاستخدام النهائي.


### اكتشاف إضافي أثناء تتبع multi-touch (قيد الإصلاح)
- في `canvas.rs::handle_presses` تستدعي اللمسة الأولى `press_at` فور وصول `TouchPhase::Start`، أي أن بعض نقاط الأوامر تُرسل للمحرك قبل معرفة ما إذا كان المستخدم سيضيف إصبعاً ثانياً.
- عند اكتشاف `multi_touch()` يمسح الكود `app.canvas.press`، لكنه لا يستطيع التراجع عن `Session::input(Input::Point(...))` الذي نُفذ بالفعل في الإطار السابق.
- النتيجة المحتملة: إذا بدأ المستخدم بلمسة واحدة ثم وضع إصبعاً ثانياً للتحريك/التكبير، قد تُحتسب النقطة الأولى رغم أن قصده كان التنقل.
- هذا ليس نفس خطأ `Touch Cancel` الذي أُصلح أعلاه؛ يتطلب تأجيل اعتماد اللمسة الأصلية على Android إلى أن يتضح أنها نقرة أحادية، أو فصل مسار Touch عن Pointer مع الحفاظ على سلوك الماوس المكتبي.
- أولوية الإصلاح: P0 قبل اعتماد تجربة الرسم باللمس. لا نغيّر سلوك desktop.


### التعديل الثاني: تأجيل اعتماد اللمسة الأولى لمنع نقطة رسم عرضية
- الملفات: `crates/ui-egui/src/canvas/press.rs` و`crates/ui-egui/src/canvas.rs`.
- أصبحت مراحل اللمس الأصلية مميزة عن Pointer/Mouse: `TouchDown/TouchMove/TouchUp/TouchCancel`.
- عند بدء لمسة واحدة على Android، يخزن التطبيق موضع البداية والحركة الأخيرة مؤقتاً لمدة `140ms` قبل إرسال النقطة إلى المحرك.
- إذا بدأ الإصبع الثاني خلال هذه المهلة، يكتشف مسار `multi_touch()` الإيماءة ويمسح اللمسة المعلقة دون إرسال نقطة CAD.
- إذا رفع المستخدم إصبعه قبل انتهاء المهلة، يعيد التطبيق بناء تسلسل Down/Move/Up بترتيبه، حتى لا تضيع النقرات السريعة.
- إذا استمرت اللمسة كإصبع واحد حتى انتهاء المهلة، يرسل Down المؤجل ثم الحركة الأخيرة.
- لا يتغير مسار Pointer/Mouse المكتبي.
- Commits: `e3b02a33874c021fd1f0f1823d3d481ba893f2a1`, `2ec6322055933d1da892e5b03c0d4e82b2e71587`, `96b4a0577be92f550905b5cace543b83e002d634`.
- **قيد يجب اختباره:** إذا أضيف الإصبع الثاني بعد انتهاء مهلة 140ms، تكون اللمسة الأولى قد اعتمدت بالفعل. لذلك لا نعتبر هذا الحل نهائياً قبل اختبار سرعة وضع الإصبعين على أجهزة حقيقية وضبط المهلة أو اعتماد آلية تراجع آمنة عند الحاجة.

### تحسين CI
- أضيفت concurrency group لإلغاء البناء القديم عند وصول commit أحدث على الفرع.
- توحيد `ANDROID_NDK_HOME` و`ANDROID_NDK_ROOT` على NDK 27.2.12479018 لتفادي اختلاف المسارين الذي ظهر في log.
- أضيف `--locked` لاختبارات Cargo وبناء Android لمنع تحديث lockfile ضمنياً في CI.
- Commit: `97b3f1f01c0808fa9b7ae9c90da6812138ec1721`.
- التشغيل الحالي المرتبط بهذا commit: https://github.com/loay94an/CAD/actions/runs/45307236220 (كان queued عند آخر تحقق).

### تصحيح نتيجة CI السابقة
- تشغيل `45306706578` فشل على commit `9845675` لأن فرع `Step::Cancel` لم يكن قد وصل بعد إلى match في `canvas.rs` عند بناء ذلك الـ commit الوسيط.
- أضيف المعالج في commit `dda6584`، ثم أضيف اختبار CI.
- لا ننسب فشل commit الوسيط إلى أحدث شجرة مصدر؛ لكن لا نسجل نجاح أحدث شجرة قبل اكتمال تشغيلها.

### حالة M2 بعد التعديل الثاني
- [x] تمييز Touch عن Pointer في event mapping.
- [x] تمييز Cancel عن End.
- [x] تأجيل Down على Android للسماح بالتعرف على multi-touch.
- [x] مسح اللمسة المعلقة عند اكتشاف إصبعين.
- [ ] اختبارات آلية لحالات pending-touch نفسها (نقرة سريعة، حركة، إلغاء، إصبع ثانٍ قبل المهلة).
- [ ] انتظار CI على commit `97b3f1f`.
- [ ] اختبار جهاز Android حقيقي وتأكيد أن الرسم لا يفقد نقاطه ولا يضيف نقطة عند pinch.


### التعديل الثالث: اختبارات مستقلة لمخزن اللمسة المؤجلة
- عُزل تحويل مراحل اللمس إلى تسلسل الضغط في `normalize_touch_steps` لتصبح له حدود واضحة وقابلة للاختبار.
- أضيفت اختبارات في `crates/ui-egui/src/canvas/tests.rs`:
  - `native_touch_tap_is_committed_on_release_without_waiting_for_delay`
  - `native_touch_move_is_buffered_until_single_finger_gesture_is_recognized`
  - `native_touch_cancel_discards_a_buffered_first_contact`
- Commit الاختبارات: `685f1211c28beb10203872e7d241b0b04b4f1a0e`.
- Commit عزل المعالج: `ca73371dc237049972217adbd2d3e82eae95423d`.
- تم تعديل مرشح CI ليشغّل كل اختبارات `canvas::` وليس فقط اختبارات `canvas::press`، حتى لا تُبنى اختبارات اللمسة المؤجلة دون تشغيلها.
- Commit توسيع الاختبار: `d6034915571c737fa6d14eecc2b4d27b45aa3d46`.

### الحالة الحالية الدقيقة
- إصلاح Cancel موجود.
- تأجيل اللمسة الأولى موجود.
- اختبارات التحويل والإلغاء والنقرة السريعة والحركة موجودة في المصدر.
- workflow يشغّل مجموعة اختبارات canvas كاملة قبل البناء.
- لم تصل بعد نتيجة CI للنسخة النهائية التي تضم الاختبارات الثلاثة؛ لا يوجد تأكيد بناء أو APK من هذه النسخة.


### ملاحظة CI بعد توسيع مرشح الاختبارات
- فشلت محاولتا التشغيل `45307439199` و`45307456044` قبل إنشاء أي job (GitHub أعاد `total_count: 0` للوظائف)، لذلك لم تنتج عنهما أخطاء Rust قابلة للتشخيص.
- كان مرشح الاختبار ينتهي بـ `canvas::` دون اقتباس؛ تم وضع أمر الاختبار بين علامتي اقتباس لتجنب أي التباس في تحليل YAML.
- Commit: `b73fdaf38532b955e44749ee602ea22fbe07c0bd`.
- تشغيل إعادة التحقق: https://github.com/loay94an/CAD/actions/runs/45307519010 (queued عند آخر تحقق).
- التشغيل `45306710894` نجح، لكنه يسبق تنفيذ تأجيل اللمسة واختباراتها؛ لذلك لا يُعد إثباتاً لسلامة أحدث تعديل.



### التعديل الرابع: حماية اللمسة المعلقة عند فقدان التركيز
- الملفات: `crates/ui-egui/src/canvas.rs`.
- عند انتقال التطبيق إلى الخلفية أو فقدان تركيز النافذة، قد لا يصل حدث TouchUp/TouchCancel المقابل.
- أضيف مسح `touch_pending` و`press` عند فقدان التركيز، مع تجاهل خطوات اللوحة في الإطار غير المركّز.
- Commits: `9df9c2600d1660de75db77e6af8755429242cafd`, `222db2cf1da97ed3f307f8e2db12a287ed640be0`.
- الهدف منع تحويل لمسة معلقة إلى نقطة CAD بعد الرجوع من الخلفية.


### مراجعة أخيرة لمسار اللمس
- أزيل `unreachable!` من فرع الحماية؛ فالمشروع يفرض قيود Clippy على panic/unreachable.
- أصبحت أي مرحلة Touch خام تصل إلى match النهائي تُسقط بأمان وتمسح اللمسة المعلقة بدلاً من إسقاط التطبيق.
- Commit: `05bfc061abcec828548da9bb057bb180650aeba5`.


### التعديل الخامس: اكتشاف الإصبع الثاني قبل اكتمال لقطة multi_touch
- الملف: `crates/ui-egui/src/canvas.rs`.
- أصبح `normalize_touch_steps` يفحص بدايات اللمس الخام قبل اعتماد اللمسة المؤجلة.
- إذا وصلت بدايتا لمس في الإطار نفسه، أو وصلت بداية جديدة بينما توجد لمسة أولى معلقة، تُلغى اللمسة الأولى وتُسقط أحداث الإطار بدلاً من تحويل أي منهما إلى نقطة CAD.
- أضيف اختبار `second_native_touch_start_cancels_the_buffered_first_contact`.
- Commits: `621983bd6439b98541db46006cca812375ddff41`, `7900e5186c6a59172664b25cb00d707928a3c40e`.
- يبقى اختبار تتابع TouchId الفعلي عبر egui وجهاز Android ضرورياً؛ هذه الحماية تعالج بدايات اللمس التي تظهر في stream، ولا تحل محل اختبار gesture recognizer على الجهاز.


### مخرج جديد من M0: فهرس أسماء أوامر المحرك
- أضيف `docs/android-command-source-index.md`، ويحتوي أسماء التصريحات الصريحة لكل ملف من 40 وحدة.
- العدد بعد إعادة التحقق من ملف `annotate.rs`: **387 تصريحاً** بلا أسماء مكررة بين هذه الملفات.
- Commit الفهرس: `adb4b63162514ddb6491ee2ef3ed2c35364c7b9c`.
- Commit تصحيح العدد وإضافة `dimspace` الذي كان ناقصاً في النسخة الأولى: `30f51a386855575582f5a2b2cc771a0ca8766457`.
- هذا يغلق جرد أسماء المصدر المباشر فقط؛ لا يغلق runtime inventory ولا يربط كل handler/parameter/test حتى الآن.
- الرابط: https://github.com/loay94an/CAD/blob/feat/android-phone-tablet/docs/android-command-source-index.md


### إغلاق جرد التسجيلات على مستوى المصدر والـ registry
- أثبت تتبع `command_specs()` أنه يضم 387 تصريحاً مباشراً + `file::alias_specs()` (أمران من `pgp.rs`) = **389 أمر محرك** وفق بنية التسجيل الحالية.
- `menus.rs::UI_COMMANDS` يحتوي **60 إجراء واجهة فقط** منفصلاً عن CommandSpec.
- أضيف إلى فهرس المصدر القسمان الخاصان بأمري PGP وإجراءات الواجهة الستين.
- أضيف اختبار في `crates/engine/src/cmd.rs` للتحقق من 389 ID وعدم التكرار.
- أضيف اختبار في `crates/ui-egui/src/menus.rs` للتحقق من 60 UI ID وعدم التكرار.
- أضيفت خطوتا الاختبار إلى Android workflow، بالإضافة إلى اختبارات canvas.
- Commits:
  - `6796438c39d5ae9691762a2b333fdcab41571b34`
  - `c7eb32b85b598c9d89fbc2f8bf547c50ccd16425`
  - `614caa6e90731d5a3479df85d3001b7a71bd5bf3`
  - `c74a7f8058e8dd130fda7f0b853c3a6e36f9073b`
- **M0 source/registry count:** اكتمل العدّ والتفرّد كاختبارات مصدر، لكن ربط handler/parameters/interactive prompts/UI reachability/Android dependencies لكل ID ما زال مفتوحاً.

---

## تنفيذ متوازٍ للمراحل — تحديث 2026-11-24

اعتماد أسلوب مسارات عمل متوازية بدلاً من إيقاف كل التطوير حتى تنتهي مرحلة واحدة. التوازي هنا على مستوى التدقيق والتغييرات المستقلة، مع دمج منضبط على الفرع نفسه وإعادة تشغيل CI بعد كل تغيير مؤثر.

### المسارات النشطة

| المسار | نطاق العمل المتزامن | بوابة الدمج |
|---|---|---|
| A — الملفات وSAF | تتبع كل عمليات القراءة والكتابة والملفات التابعة وواجهات المضيف؛ بناء تصميم URI/streams قبل كتابة الجسر | فتح/حفظ/حفظ باسم/تصدير عبر DocumentProvider مع صلاحيات مستمرة واختبارات فشل |
| B — اللمس والرسم | اختبارات Touch/Pointer/pinch/cancel وتفاعل الأوامر | اختبارات canvas ناجحة ثم تحقق على جهاز Android |
| C — الهاتف والتابلت | قياسات العرض، dock/panels، command console، IME، landscape | اختبار widths صغيرة وكبيرة مع عدم إخفاء أي أداة |
| D — الأوامر واللوحات | ربط command registry وUI_COMMANDS بالـ handlers والـ params والـ prompts والاختبارات | لا يبقى أمر مسجل بلا مصدر تنفيذ وحالة اختبار موثقة |
| E — الصيغ والتبعيات | DXF/DWG وPDF/SVG/PNG والصور/Xrefs/fonts/LISP/PGP | round-trip واختبارات ARM64 وملفات مرجعية |
| F — العربية والأداء | اكتمال catalog، النص المختلط RTL/LTR، الخطوط، أحجام الملفات والذاكرة | اختبار عربي فعلي، profiling وstress tests |

### تقدم منفذ ضمن المسار C / M5

- الملف: `crates/ui-egui/src/cmdline.rs`.
- كان عرض سطر الأوامر يستخدم حدّاً أدنى قدره 360 نقطة؛ هذا أكبر من مساحة بعض شاشات الهاتف، وقد يجعل عرض الشريط صفراً أو يضيّق عناصره بشدة.
- أضيف حساب عرض متجاوب خاص بـ Android: حتى 94% من مساحة الرسم مع هامش 8 نقاط، مع إبقاء سلوك desktop كما هو.
- أضيف اختبار انحدار يقارن العرض المضغوط بعرض desktop على مساحة 300 نقطة.
- Commit: `b19bf5609abb74e8c6ca9ca5bc4daf673ebddab4`.
- هذه خطوة محدودة داخل M5/M3 وليست إغلاقاً للمرحلتين؛ يلزم CI واختبار IME على جهاز حقيقي.

### قيود تمنع ادعاء اكتمال SAF مبكراً

- واجهة `Services` الحالية تعيد أسماء مسارات نصية، بينما SAF يعيد `content:// URI` لا يمثل `Path`.
- `read_file` و`store` في المحرك يقرآن ويكتبان من نظام الملفات ويعتمدان على فحوصات المسارات والكتابة الذرية والنسخ الاحتياطي.
- `IoHooks` الحالية توفر فك/ترميز bytes، لكنها لا تغطي وحدها فتح المستندات وحفظها أو مراجع الصور وXrefs والخطوط.
- لذلك لا يجوز توصيل منتقي Android وإرجاع URI كسلسلة مسار؛ يلزم تصميم DocumentRef/host I/O متكامل أو طبقة staging دائمة ذات mapping وصلاحيات SAF، مع اختبارات إعادة التشغيل والحفظ الآمن.

### بوابة التنفيذ التالية

تستمر مسارات التدقيق بالتوازي، ويكون دمج SAF بعد حسم عقدة التخزين والـ URI، بينما تستمر اختبارات اللمس وتصميم الهاتف/التابلت وفهرسة handlers دون انتظار ذلك. لا تُسجّل أي مرحلة مكتملة إلا بعد نجاح اختبارها وبوابة التكامل المناسبة.
