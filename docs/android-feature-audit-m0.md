# CADCraft Android — سجل التدقيق الوظيفي الأولي (M0)

**المستودع:** loay94an/CAD  
**الفرع:** feat/android-phone-tablet  
**مرجع التدقيق:** 31446f8bbc09a72b5fed7a76b9634e7e2bd4605d  
**تاريخ الفحص:** 2026-11-24  
**الحالة:** تدقيق ساكن موسع؛ ليس اختبار قبول على جهاز Android.

## 1. نطاق ما تم فحصه

تم جلب شجرة Git كاملة دون truncation عند المرجع أعلاه:
- 1,414 مساراً في الشجرة.
- 19 مساراً تحت apps/cadcraft-android.
- 107 مسارات تحت crates/ui-egui/src.
- 277 مساراً تحت crates/engine/src.
- 193 مساراً تحت crates/io.
- 595 مساراً تطابق أسماءها أنماط test/tests/spec في المستودع؛ هذا عدد مسارات لا عدد حالات اختبار منفذة.
- 39 مساراً تحت docs.

تمت قراءة ملفات Android entry point وManifest/Gradle ومصادر UI المحورية: cmdline.rs وopening.rs وexchange.rs وdocking.rs وmenus.rs وi18n tests، وأوامر الملفات في engine، وCargo features، ووثائق gaps/file-format-parity/ui-parity/localization-parity/parity-checklist.

لم تُقرأ كل 1,414 مساراً سطراً بسطر ولم تُنفذ جميع اختبارات workspace. لذلك لا يوصف هذا السجل بأنه تدقيق شامل لكل أمر أو اختبار سلوكي.

## 2. نتائج مثبتة من المصدر

| المجال | دليل المصدر | الحالة الحالية على Android | الإجراء المطلوب |
|---|---|---|---|
| تشغيل التطبيق | apps/cadcraft-android/src/lib.rs ينشئ eframe NativeOptions مع AndroidApp | مضيف أصلي موجود | اختبار lifecycle والأجهزة |
| إعادة استخدام المحرك | AndroidShell يحتوي CadApp ويستدعي logic/ui | يعيد استخدام محرك CAD الأصلي | الحفاظ على هذا المسار |
| خدمات المنصة | CadApp::new(Session::new(), Services::default()) | لا يوجد Android picker في نقطة الإنشاء | تنفيذ Services Android |
| منتقي الملفات | تعليق install_io يقول صراحة إن file picking مؤجل إلى SAF bridge لاحق | غير موصول في المضيف | SAF Open/Create Document |
| codecs | install_io يثبت read/write/plot/image decoder/write_as/entity/object/proxy/DWG hooks | codecs مثبتة في المضيف | اختبار كل codec من Android عبر bytes |
| فتح الملفات | opening.rs يستخدم Path/PathBuf وread_file(path) وworker thread | مسار ملف محلي، لا content URI | abstraction للـ URI/stream |
| استيراد/تصدير/نشر | exchange.rs يستدعي pick_then(Pick::Open/Save) | يعتمد على خدمات picker | ربط SAF واختبار cancellation |
| معاينة الرسم | exchange.rs يولد PNG ويحمّله كـ TextureHandle، نافذة Preview بأزرار Print/Close | المعاينة موجودة؛ Print يستدعي plot | اختبار ملاءمة الشاشة والطباعة والتصدير |
| سطر الأوامر | cmdline.rs يدعم history/recent input/expanded/IME/preedit/Tab/autocomplete/dynamic input | ميزات قوية موجودة ضمن floating command line | تخطيط Mobile bottom console واختبار IME |
| اللوحات | docking.rs يعرف Canvas/ToolSets/QuickCalc/Layers/Properties/ExternalRefs/Blocks | نظام Dock موجود ومهيأ بأبعاد Android لبعض الأعمدة | استبدال/تكييف dock للهاتف، إبقاؤه للتابلت |
| القوائم | menus.rs يبني شجرة مرجعية كبيرة للقوائم ويجمع عناصر UI | القوائم مكتبية بالأساس | جرد كل menu action وربطه بشاشة هاتف |
| العربية | ar.tsv يحوي 1,417 سطراً تقريباً و1,408 صفوف بيانات بحسب العد النصي | قاموس واسع موجود | استخراج missing/fallback واختبار bidi/IME؛ عدد الصفوف لا يساوي نسبة اكتمال |
| الاختبارات | ملفات tests كثيرة في engine/io/ui | تغطية محرك كبيرة، لكن workflow Android الذي راجعناه ينفذ build/package validation فقط | فصل نتائج unit/host/android/device |

## 3. نتيجة مهمة: ما هو متاح في المكتبات لا يساوي متاحاً في Android

- `cadcraft-io` يعلن عن DXF وSVG وPNG وPDF في وصف الحزمة ويحتوي على DWG bridge عبر dependency.
- مضيف Android يثبت read/write/plot hooks.
- لكن picker والوصول إلى مستندات المستخدم عبر SAF غير موصول في المضيف بحسب التعليق والكود الحالي.
- لذلك لا نعلن اكتمال Open/Save/Export لمجرد وجود codecs.
- دعم DWG لا يُعلن كميزة Android حتى ينجح اختبار القراءة/الكتابة والاعتماديات على ARM64 على ملفات حقيقية.

## 4. نتيجة CI التي تم التحقق منها

Workflow run: https://github.com/loay94an/CAD/actions/runs/45301638082  
Commit: 59cff36462dda197a434f55835d2f4e327df890a  
الحالة: success.

الـ job الوحيد المعروض هو build-arm64، وخطواته تشمل إعداد Rust/Java/Android NDK، بناء المكتبة الأصلية، Assemble APK، Validate APK package، ورفع artifact.

**ما لا يثبته هذا التشغيل:** لا يثبت أن التطبيق يفتح على هاتف، أو أن SAF يعمل، أو أن كل الأوامر تعمل باللمس، أو أن التصدير يفتح الناتج في تطبيق خارجي، أو أن الأداء مقبول. كما لا يثبت أن workspace test suite شغلت في هذا workflow.

## 5. فجوات وظيفية/مخاطر مثبتة

### P0 — ملفات Android
1. تنفيذ Android file picker وCreate Document.
2. تصميم abstraction يتعامل مع URI وContentResolver streams بدلاً من تحويل URI إلى Path.
3. فتح الملف في worker من stream مع حدود قراءة واضحة.
4. حفظ ذري/آمن وإبقاء dirty state عند الفشل.
5. ربط Import/Export/Publish/Plot إلى SAF.
6. دعم الملفات التابعة: صور، خطوط، xrefs، scripts، templates، وبلوكات خارجية حسب مسارات المحرك.
7. اختبار مزودي الملفات وإلغاء الاختيار وفشل الكتابة واستعادة الصلاحيات.

### P1 — الإدخال وسطر الأوامر
1. cmdline الحالي floating؛ يحتاج bottom sheet/console متكيف.
2. لوحة المفاتيح تغطي مساحة العمل؛ يلزم IME insets وتصغير/طي تلقائي.
3. التحقق من composition/preedit وTab/Enter/Escape/History والاقتراحات على Android.
4. اختبار العربية والإنجليزية والإحداثيات والأرقام وعلامات الترقيم.

### P2 — الرسم واللمس
1. إكمال state machine لأحداث Touch/Pointer ومنع ازدواج الأحداث.
2. اختبار end/release يثبت العنصر مرة واحدة.
3. اختبار multi-touch pan/pinch أثناء الأوامر.
4. اختبار hit targets وgrips وselection windows وsnap.
5. اختبار stylus إن توفرت الأجهزة.

### P3 — واجهة الهاتف والتابلت
1. الهاتف: مساحة رسم + top bar + bottom tool groups + command console + sheets.
2. التابلت: dock panels متعددة مع keyboard/stylus.
3. كل عنصر من menus/catalog يجب أن يربط بوظيفة أو يظهر disabled بوضوح.
4. مراجعة كل palette وdialog لمنع clipping والتداخل.
5. اختبار portrait/landscape وكثافات الشاشة.

### P4 — الصيغ والتصدير
1. جرد فعلي لكل تنسيق مدعوم بحسب Cargo cfg/feature والـ writer.
2. round-trip DXF وملفات واقعية مرخصة.
3. PDF/PNG/SVG export مع معاينة ومقاسات ورق.
4. DWG read/write ARM64 وحفظ المحتوى غير المعروف؛ لا دعم معلن قبل اجتياز الاختبار.
5. التحقق من عدم فقد بيانات المستند.

### P5 — العربية وRTL
1. استخراج فجوات ar.tsv باستخدام تقرير i18n المخصص.
2. اختبار اتجاه الحقول المختلطة LTR/RTL.
3. التأكد من تشكيل النص العربي في canvas والتصدير.
4. مراجعة accessibility labels.

### P6 — الاستقرار والأداء
1. lifecycle pause/resume/rotation/process death.
2. GPU context loss وذاكرة الرسومات الكبيرة.
3. قياس frame times واستهلاك الذاكرة والحرارة.
4. اختبار autosave/recovery وعدم فقد المستند.

## 6. خريطة الاختبارات المطلوبة وسجل النتائج

كل اختبار يسجل: ID، الجهاز/ABI، Android API، حجم الشاشة/density، commit/APK SHA256، الخطوات، النتيجة، logcat، واللقطة إن أمكن.

| ID | السيناريو | نوع الاختبار | الحالة |
|---|---|---|---|
| A-001 | تثبيت وتشغيل أول مرة ARM64 | جهاز فعلي | لم ينفذ |
| A-002 | pause/resume وتدوير الشاشة | جهاز فعلي | لم ينفذ |
| F-001 | Open عبر SAF من Downloads | جهاز فعلي | غير موصول بعد |
| F-002 | Save As عبر SAF | جهاز فعلي | غير موصول بعد |
| F-003 | حفظ ثم إغلاق ثم إعادة فتح DXF | جهاز فعلي | لم ينفذ |
| F-004 | Export PDF/PNG/SVG | جهاز فعلي | لم ينفذ |
| F-005 | إلغاء picker وفشل مزود الملفات | جهاز فعلي | لم ينفذ |
| C-001 | إدخال LINE عبر command line | UI/device | لم ينفذ |
| C-002 | Tab autocomplete/history/keywords | UI/device | لم ينفذ |
| T-001 | سحب خط ثم release ثم ثباته | جهاز فعلي | يحتاج اختبار المستخدم |
| T-002 | تحديد الخط وتحريكه وتعديل grips | جهاز فعلي | لم ينفذ |
| T-003 | pinch/pan أثناء الرسم | جهاز فعلي | لم ينفذ |
| T-004 | snap/ortho/grid | جهاز فعلي | لم ينفذ |
| U-001 | الهاتف portrait/landscape بلا clipping | جهاز فعلي | لم ينفذ |
| U-002 | تابلت واللوحات المتعددة | جهاز فعلي | لم ينفذ |
| L-001 | إدخال عربي/إنجليزي مختلط | جهاز فعلي | لم ينفذ |
| P-001 | ملف كبير وقياس الأداء | جهاز فعلي | لم ينفذ |

## 7. القرار التنفيذي

المرحلة M0 لم تغلق بعد. الخطوة التالية ليست إضافة مزيد من الأزرار؛ بل:
1. بناء جرد آلي للأوامر المسجلة من command registry وربطه بعناصر القوائم والأدوات.
2. تتبع كل مجموعة أمر إلى prompt/handler/commit/cancel واختبار موجود.
3. استخراج كل استعمالات خدمات الملفات وfilesystem في UI/engine وتحديد ما يتطلب Android bridge.
4. إنشاء Android SAF bridge وخدماته ثم كتابة اختبارات.
5. توسيع CI ليشمل host tests المناسبة وAndroid package checks، ثم الاختبار على أجهزة حقيقية.

## 8. تعريف الإغلاق

لا تعتبر M0 مكتملة إلا عندما يكون لكل أمر/لوحة/قائمة/تنسيق:
- مصدر تنفيذ محدد.
- تصنيف اعتماديات المنصة.
- حالة Android مع دليل.
- اختبار أو سبب موثق لعدم إمكان الاختبار.
- فجوة/إصلاح وأولوية.

ولا تعتبر عملية التحويل مكتملة إلا بعد اجتياز اختبارات الجهاز الفعلية المذكورة في هذا السجل.
