#[cfg(feature = "android")]
use jni::{
    objects::{JClass, JObject, JString, JValue, JValueGen},
    sys::{jboolean, jint, jlong, jstring},
    JNIEnv,
};
#[cfg(feature = "android")]
use std::sync::OnceLock;

#[cfg(feature = "android")]
static JVM: OnceLock<jni::JavaVM> = OnceLock::new();

#[cfg(feature = "android")]
static CONTEXT: OnceLock<jni::objects::GlobalRef> = OnceLock::new();

#[cfg(feature = "android")]
pub fn init_jvm(vm: jni::JavaVM) {
    let _ = JVM.set(vm);
}

#[cfg(feature = "android")]
pub fn get_jvm() -> Option<&'static jni::JavaVM> {
    JVM.get()
}

#[cfg(feature = "android")]
pub fn set_context(ctx: jni::objects::GlobalRef) {
    let _ = CONTEXT.set(ctx);
}

#[cfg(feature = "android")]
pub fn get_context<'a>() -> Option<jni::objects::JObject<'a>> {
    CONTEXT.get().map(|r| unsafe { jni::objects::JObject::from_raw(r.as_obj().as_raw()) })
}

#[cfg(feature = "android")]
pub fn with_env<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut JNIEnv) -> Result<R, jni::errors::Error>,
{
    let jvm = get_jvm()?;
    let mut env = jvm.get_env().ok()?;
    f(&mut env).ok()
}

#[cfg(feature = "android")]
pub fn with_env_opt<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut JNIEnv) -> Option<R>,
{
    let jvm = get_jvm()?;
    let mut env = jvm.get_env().ok()?;
    f(&mut env)
}

pub trait JValueConvert<'a> {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self;
}

impl<'a> JValueConvert<'a> for i32 {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.i().unwrap_or(0)
    }
}

impl<'a> JValueConvert<'a> for bool {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.z().unwrap_or(false)
    }
}

impl<'a> JValueConvert<'a> for f32 {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.f().unwrap_or(0.0)
    }
}

impl<'a> JValueConvert<'a> for f64 {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.d().unwrap_or(0.0)
    }
}

impl<'a> JValueConvert<'a> for i64 {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.j().unwrap_or(0)
    }
}

impl<'a> JValueConvert<'a> for () {
    fn from_jvalue(_val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        ()
    }
}

impl<'a> JValueConvert<'a> for JObject<'a> {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        val.l().unwrap_or_else(|_| jni::objects::JObject::null())
    }
}

impl<'a> JValueConvert<'a> for JString<'a> {
    fn from_jvalue(val: jni::objects::JValueGen<jni::objects::JObject<'a>>) -> Self {
        let obj = val.l().unwrap_or_else(|_| jni::objects::JObject::null());
        unsafe { std::mem::transmute(obj) }
    }
}
pub fn call_static_method<'a, T>(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue<'a, '_>],
) -> Option<T>
where
    T: JValueConvert<'a>,
{
    let res = with_env(|env| {
        let class = env.find_class(class_name)?;
        let result = env.call_static_method(class, method_name, sig, args)?;
        Ok(unsafe { std::mem::transmute(result) })
    });
    res.map(|v| T::from_jvalue(v))
}

#[cfg(feature = "android")]
pub fn call_method<'a, T>(
    obj: &JObject<'a>,
    method_name: &str,
    sig: &str,
    args: &[JValue<'a, '_>],
) -> Option<T>
where
    T: JValueConvert<'a>,
{
    let res = with_env(|env| {
        let result = env.call_method(obj, method_name, sig, args)?;
        Ok(unsafe { std::mem::transmute(result) })
    });
    res.map(|v| T::from_jvalue(v))
}

#[cfg(feature = "android")]
pub fn get_field<'a, T>(obj: &JObject<'a>, field_name: &str, sig: &str) -> Option<T>
where
    T: JValueConvert<'a>,
{
    let res = with_env(|env| {
        let result = env.get_field(obj, field_name, sig)?;
        Ok(unsafe { std::mem::transmute(result) })
    });
    res.map(|v| T::from_jvalue(v))
}

#[cfg(feature = "android")]
pub fn set_field<'a>(obj: &JObject<'a>, field_name: &str, sig: &str, value: JValue<'a, '_>) -> Option<()> {
    with_env(|env| {
        env.set_field(obj, field_name, sig, value)?;
        Ok(())
    })
}

#[cfg(feature = "android")]
pub fn new_string<'a>(s: &str) -> Option<JString<'a>> {
    let res = with_env(|env| {
        let js = env.new_string(s)?;
        Ok(unsafe { std::mem::transmute::<JString<'_>, JString<'static>>(js) })
    });
    res.map(|js| unsafe { std::mem::transmute(js) })
}

#[cfg(feature = "android")]
pub fn get_string<'a>(jstr: &JString<'a>) -> Option<String> {
    with_env(|env| {
        let s: String = env.get_string(jstr)?.into();
        Ok(s)
    })
}

#[cfg(feature = "android")]
pub fn find_class<'a>(name: &str) -> Option<JClass<'a>> {
    let res = with_env(|env| {
        let cls = env.find_class(name)?;
        Ok(unsafe { std::mem::transmute::<JClass<'_>, JClass<'static>>(cls) })
    });
    res.map(|cls| unsafe { std::mem::transmute(cls) })
}

#[cfg(feature = "android")]
pub fn new_object<'a>(class_name: &str, sig: &str, args: &[JValue<'a, '_>]) -> Option<JObject<'a>> {
    let res = with_env(|env| {
        let class = env.find_class(class_name)?;
        let obj = env.new_object(class, sig, args)?;
        Ok(unsafe { JObject::from_raw(obj.into_raw()) })
    });
    res
}

#[cfg(feature = "android")]
pub fn call_static_object_method<'a>(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue<'a, '_>],
) -> Option<JObject<'a>> {
    let res = with_env(|env| {
        let class = env.find_class(class_name)?;
        let result = env.call_static_method(class, method_name, sig, args)?;
        let obj = result.l()?;
        Ok(unsafe { JObject::from_raw(obj.into_raw()) })
    });
    res
}

#[cfg(feature = "android")]
pub fn call_static_int_method(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue],
) -> Option<i32> {
    call_static_method(class_name, method_name, sig, args)
}

#[cfg(feature = "android")]
pub fn call_static_boolean_method(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue],
) -> Option<bool> {
    call_static_method(class_name, method_name, sig, args)
}

#[cfg(feature = "android")]
pub fn call_static_long_method(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue],
) -> Option<i64> {
    call_static_method(class_name, method_name, sig, args)
}

#[cfg(feature = "android")]
pub fn call_static_string_method(
    class_name: &str,
    method_name: &str,
    sig: &str,
    args: &[JValue],
) -> Option<String> {
    call_static_object_method(class_name, method_name, sig, args)
        .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
}

pub fn new_global_ref<'a>(obj: &JObject<'a>) -> Option<JObject<'a>> {
    let res = with_env(|env| {
        let gref = env.new_global_ref(obj)?;
        let raw = gref.as_obj().as_raw();
        Ok(unsafe { JObject::from_raw(raw) })
    });
    res.map(|jo| unsafe { std::mem::transmute(jo) })
}

#[cfg(feature = "android")]
pub fn delete_global_ref<'a>(obj: &JObject<'a>) -> Option<()> {
    with_env(|env| {
        let raw_env = env.get_native_interface();
        unsafe {
            (**raw_env).DeleteGlobalRef.unwrap()(raw_env, obj.as_raw());
        }
        Ok(())
    })
}

#[cfg(feature = "android")]
pub fn exception_check() -> bool {
    with_env(|env| env.exception_check()).unwrap_or(false)
}

#[cfg(feature = "android")]
pub fn exception_clear() -> Option<()> {
    with_env(|env| env.exception_clear())
}

#[cfg(feature = "android")]
pub fn exception_describe() -> Option<()> {
    with_env(|env| env.exception_describe())
}

#[cfg(feature = "android")]
pub mod android {
    use super::*;
    use jni::objects::JObject;

    pub const ACTIVITY_CLASS: &str = "android/app/Activity";
    pub const CONTEXT_CLASS: &str = "android/content/Context";
    pub const INTENT_CLASS: &str = "android/content/Intent";
    pub const BUNDLE_CLASS: &str = "android/os/Bundle";
    pub const VIEW_CLASS: &str = "android/view/View";
    pub const TEXT_VIEW_CLASS: &str = "android/widget/TextView";
    pub const BUTTON_CLASS: &str = "android/widget/Button";
    pub const EDIT_TEXT_CLASS: &str = "android/widget/EditText";
    pub const LINEAR_LAYOUT_CLASS: &str = "android/widget/LinearLayout";
    pub const RELATIVE_LAYOUT_CLASS: &str = "android/widget/RelativeLayout";
    pub const CONSTRAINT_LAYOUT_CLASS: &str = "androidx/constraintlayout/widget/ConstraintLayout";
    pub const RECYCLER_VIEW_CLASS: &str = "androidx/recyclerview/widget/RecyclerView";
    pub const TOAST_CLASS: &str = "android/widget/Toast";
    pub const LOG_CLASS: &str = "android/util/Log";
    pub const VIBRATOR_CLASS: &str = "android/os/Vibrator";
    pub const POWER_MANAGER_CLASS: &str = "android/os/PowerManager";
    pub const CONNECTIVITY_MANAGER_CLASS: &str = "android/net/ConnectivityManager";
    pub const LOCATION_MANAGER_CLASS: &str = "android/location/LocationManager";
    pub const SENSOR_MANAGER_CLASS: &str = "android/hardware/SensorManager";
    pub const CAMERA_MANAGER_CLASS: &str = "android/hardware/camera2/CameraManager";
    pub const BLUETOOTH_ADAPTER_CLASS: &str = "android/bluetooth/BluetoothAdapter";
    pub const NFC_ADAPTER_CLASS: &str = "android/nfc/NfcAdapter";
    pub const TELEPHONY_MANAGER_CLASS: &str = "android/telephony/TelephonyManager";
    pub const AUDIO_MANAGER_CLASS: &str = "android/media/AudioManager";
    pub const MEDIA_PLAYER_CLASS: &str = "android/media/MediaPlayer";
    pub const MEDIA_RECORDER_CLASS: &str = "android/media/MediaRecorder";
    pub const SHARED_PREFERENCES_CLASS: &str = "android/content/SharedPreferences";
    pub const CONTENT_RESOLVER_CLASS: &str = "android/content/ContentResolver";
    pub const CURSOR_CLASS: &str = "android/database/Cursor";
    pub const SQLITE_DATABASE_CLASS: &str = "android/database/sqlite/SQLiteDatabase";
    pub const ASYNC_TASK_CLASS: &str = "android/os/AsyncTask";

    pub fn get_context<'a>() -> Option<JObject<'a>> {
        super::get_context()
    }
    pub const HANDLER_CLASS: &str = "android/os/Handler";
    pub const LOOPER_CLASS: &str = "android/os/Looper";
    pub const MESSAGE_CLASS: &str = "android/os/Message";
    pub const RUNNABLE_CLASS: &str = "java/lang/Runnable";
    pub const THREAD_CLASS: &str = "java/lang/Thread";
    pub const EXECUTOR_SERVICE_CLASS: &str = "java/util/concurrent/ExecutorService";
    pub const EXECUTORS_CLASS: &str = "java/util/concurrent/Executors";
    pub const FUTURE_CLASS: &str = "java/util/concurrent/Future";
    pub const CALLABLE_CLASS: &str = "java/util/concurrent/Callable";
    pub const ARRAY_LIST_CLASS: &str = "java/util/ArrayList";
    pub const HASH_MAP_CLASS: &str = "java/util/HashMap";
    pub const HASH_SET_CLASS: &str = "java/util/HashSet";
    pub const STRING_CLASS: &str = "java/lang/String";
    pub const INTEGER_CLASS: &str = "java/lang/Integer";
    pub const LONG_CLASS: &str = "java/lang/Long";
    pub const BOOLEAN_CLASS: &str = "java/lang/Boolean";
    pub const DOUBLE_CLASS: &str = "java/lang/Double";
    pub const OBJECT_CLASS: &str = "java/lang/Object";
    pub const CLASS_CLASS: &str = "java/lang/Class";
    pub const METHOD_CLASS: &str = "java/lang/reflect/Method";
    pub const FIELD_CLASS: &str = "java/lang/reflect/Field";
    pub const CONSTRUCTOR_CLASS: &str = "java/lang/reflect/Constructor";
    pub const MODIFIER_CLASS: &str = "java/lang/reflect/Modifier";
    pub const SYSTEM_CLASS: &str = "java/lang/System";
    pub const RUNTIME_CLASS: &str = "java/lang/Runtime";
    pub const PROCESS_CLASS: &str = "android/os/Process";
    pub const BUILD_CLASS: &str = "android/os/Build";
    pub const VERSION_CLASS: &str = "android/os/Build$VERSION";
    pub const ENVIRONMENT_CLASS: &str = "android/os/Environment";
    pub const FILE_CLASS: &str = "java/io/File";
    pub const FILE_INPUT_STREAM_CLASS: &str = "java/io/FileInputStream";
    pub const FILE_OUTPUT_STREAM_CLASS: &str = "java/io/FileOutputStream";
    pub const BUFFERED_READER_CLASS: &str = "java/io/BufferedReader";
    pub const INPUT_STREAM_READER_CLASS: &str = "java/io/InputStreamReader";
    pub const OUTPUT_STREAM_WRITER_CLASS: &str = "java/io/OutputStreamWriter";
    pub const JSON_OBJECT_CLASS: &str = "org/json/JSONObject";
    pub const JSON_ARRAY_CLASS: &str = "org/json/JSONArray";
    pub const HTTP_URL_CONNECTION_CLASS: &str = "java/net/HttpURLConnection";
    pub const URL_CLASS: &str = "java/net/URL";
    pub const URI_CLASS: &str = "java/net/URI";
    pub const SSL_CONTEXT_CLASS: &str = "javax/net/ssl/SSLContext";
    pub const TRUST_MANAGER_FACTORY_CLASS: &str = "javax/net/ssl/TrustManagerFactory";
    pub const KEY_MANAGER_FACTORY_CLASS: &str = "javax/net/ssl/KeyManagerFactory";
    pub const KEY_STORE_CLASS: &str = "java/security/KeyStore";
    pub const CERTIFICATE_FACTORY_CLASS: &str = "java/security/cert/CertificateFactory";
    pub const X509_CERTIFICATE_CLASS: &str = "java/security/cert/X509Certificate";
    pub const MESSAGE_DIGEST_CLASS: &str = "java/security/MessageDigest";
    pub const CIPHER_CLASS: &str = "javax/crypto/Cipher";
    pub const SECRET_KEY_SPEC_CLASS: &str = "javax/crypto/spec/SecretKeySpec";
    pub const IV_PARAMETER_SPEC_CLASS: &str = "javax/crypto/spec/IvParameterSpec";
    pub const MAC_CLASS: &str = "javax/crypto/Mac";
    pub const SIGNATURE_CLASS: &str = "java/security/Signature";
    pub const KEY_PAIR_GENERATOR_CLASS: &str = "java/security/KeyPairGenerator";
    pub const KEY_GENERATOR_CLASS: &str = "javax/crypto/KeyGenerator";
    pub const SECURE_RANDOM_CLASS: &str = "java/security/SecureRandom";
    pub const BASE64_CLASS: &str = "android/util/Base64";
    pub const BITMAP_CLASS: &str = "android/graphics/Bitmap";
    pub const BITMAP_FACTORY_CLASS: &str = "android/graphics/BitmapFactory";
    pub const CANVAS_CLASS: &str = "android/graphics/Canvas";
    pub const PAINT_CLASS: &str = "android/graphics/Paint";
    pub const COLOR_CLASS: &str = "android/graphics/Color";
    pub const MATRIX_CLASS: &str = "android/graphics/Matrix";
    pub const RECT_CLASS: &str = "android/graphics/Rect";
    pub const POINT_CLASS: &str = "android/graphics/Point";
    pub const POINT_F_CLASS: &str = "android/graphics/PointF";
    pub const PATH_CLASS: &str = "android/graphics/Path";
    pub const REGION_CLASS: &str = "android/graphics/Region";
    pub const DRAWABLE_CLASS: &str = "android/graphics/drawable/Drawable";
    pub const BITMAP_DRAWABLE_CLASS: &str = "android/graphics/drawable/BitmapDrawable";
    pub const SHAPE_DRAWABLE_CLASS: &str = "android/graphics/drawable/shapes/ShapeDrawable";
    pub const GRADIENT_DRAWABLE_CLASS: &str = "android/graphics/drawable/GradientDrawable";
    pub const LAYOUT_INFLATER_CLASS: &str = "android/view/LayoutInflater";
    pub const MENU_CLASS: &str = "android/view/Menu";
    pub const MENU_ITEM_CLASS: &str = "android/view/MenuItem";
    pub const SUB_MENU_CLASS: &str = "android/view/SubMenu";
    pub const CONTEXT_MENU_CLASS: &str = "android/view/ContextMenu";
    pub const ACTION_BAR_CLASS: &str = "android/app/ActionBar";
    pub const TOOLBAR_CLASS: &str = "androidx/appcompat/widget/Toolbar";
    pub const APP_COMPAT_ACTIVITY_CLASS: &str = "androidx/appcompat/app/AppCompatActivity";
    pub const FRAGMENT_CLASS: &str = "androidx/fragment/app/Fragment";
    pub const FRAGMENT_MANAGER_CLASS: &str = "androidx/fragment/app/FragmentManager";
    pub const FRAGMENT_TRANSACTION_CLASS: &str = "androidx/fragment/app/FragmentTransaction";
    pub const VIEW_PAGER_CLASS: &str = "androidx/viewpager/widget/ViewPager";
    pub const VIEW_PAGER2_CLASS: &str = "androidx/viewpager2/widget/ViewPager2";
    pub const TAB_LAYOUT_CLASS: &str = "com/google/android/material/tabs/TabLayout";
    pub const NAVIGATION_VIEW_CLASS: &str = "com/google/android/material/navigation/NavigationView";
    pub const FLOATING_ACTION_BUTTON_CLASS: &str = "com/google/android/material/floatingactionbutton/FloatingActionButton";
    pub const SNACKBAR_CLASS: &str = "com/google/android/material/snackbar/Snackbar";
    pub const COORDINATOR_LAYOUT_CLASS: &str = "androidx/coordinatorlayout/widget/CoordinatorLayout";
    pub const APP_BAR_LAYOUT_CLASS: &str = "com/google/android/material/appbar/AppBarLayout";
    pub const COLLAPSING_TOOLBAR_LAYOUT_CLASS: &str = "com/google/android/material/appbar/CollapsingToolbarLayout";
    pub const MATERIAL_BUTTON_CLASS: &str = "com/google/android/material/button/MaterialButton";
    pub const TEXT_INPUT_LAYOUT_CLASS: &str = "com/google/android/material/textfield/TextInputLayout";
    pub const TEXT_INPUT_EDIT_TEXT_CLASS: &str = "com/google/android/material/textfield/TextInputEditText";
    pub const CHIP_CLASS: &str = "com/google/android/material/chip/Chip";
    pub const CHIP_GROUP_CLASS: &str = "com/google/android/material/chip/ChipGroup";
    pub const CARD_VIEW_CLASS: &str = "androidx/cardview/widget/CardView";
    pub const LINEAR_LAYOUT_MANAGER_CLASS: &str = "androidx/recyclerview/widget/LinearLayoutManager";
    pub const GRID_LAYOUT_MANAGER_CLASS: &str = "androidx/recyclerview/widget/GridLayoutManager";
    pub const STAGGERED_GRID_LAYOUT_MANAGER_CLASS: &str = "androidx/recyclerview/widget/StaggeredGridLayoutManager";
    pub const ITEM_TOUCH_HELPER_CLASS: &str = "androidx/recyclerview/widget/ItemTouchHelper";
    pub const DIVIDER_ITEM_DECORATION_CLASS: &str = "androidx/recyclerview/widget/DividerItemDecoration";
    pub const DEFAULT_ITEM_ANIMATOR_CLASS: &str = "androidx/recyclerview/widget/DefaultItemAnimator";
    pub const SWIPE_REFRESH_LAYOUT_CLASS: &str = "androidx/swiperefreshlayout/widget/SwipeRefreshLayout";
    pub const NESTED_SCROLL_VIEW_CLASS: &str = "androidx/core/widget/NestedScrollView";
    pub const MOTION_LAYOUT_CLASS: &str = "androidx/constraintlayout/motion/widget/MotionLayout";
    pub const CONSTRAINT_SET_CLASS: &str = "androidx/constraintlayout/widget/ConstraintSet";
    pub const MOTION_SCENE_CLASS: &str = "androidx/constraintlayout/motion/widget/MotionScene";
    pub const TRANSITION_CLASS: &str = "android/transition/Transition";
    pub const CHANGE_BOUNDS_CLASS: &str = "android/transition/ChangeBounds";
    pub const FADE_CLASS: &str = "android/transition/Fade";
    pub const SLIDE_CLASS: &str = "android/transition/Slide";
    pub const EXPLODE_CLASS: &str = "android/transition/Explode";
    pub const TRANSITION_MANAGER_CLASS: &str = "android/transition/TransitionManager";
    pub const SCENE_CLASS: &str = "android/transition/Scene";
    pub const ANIMATOR_CLASS: &str = "android/animation/Animator";
    pub const VALUE_ANIMATOR_CLASS: &str = "android/animation/ValueAnimator";
    pub const OBJECT_ANIMATOR_CLASS: &str = "android/animation/ObjectAnimator";
    pub const ANIMATOR_SET_CLASS: &str = "android/animation/AnimatorSet";
    pub const TIME_INTERPOLATOR_CLASS: &str = "android/animation/TimeInterpolator";
    pub const ACCELERATE_DECELERATE_INTERPOLATOR_CLASS: &str = "android/view/animation/AccelerateDecelerateInterpolator";
    pub const ACCELERATE_INTERPOLATOR_CLASS: &str = "android/view/animation/AccelerateInterpolator";
    pub const DECELERATE_INTERPOLATOR_CLASS: &str = "android/view/animation/DecelerateInterpolator";
    pub const LINEAR_INTERPOLATOR_CLASS: &str = "android/view/animation/LinearInterpolator";
    pub const BOUNCE_INTERPOLATOR_CLASS: &str = "android/view/animation/BounceInterpolator";
    pub const OVERSHOOT_INTERPOLATOR_CLASS: &str = "android/view/animation/OvershootInterpolator";
    pub const ANTICIPATE_INTERPOLATOR_CLASS: &str = "android/view/animation/AnticipateInterpolator";
    pub const ANTICIPATE_OVERSHOOT_INTERPOLATOR_CLASS: &str = "android/view/animation/AnticipateOvershootInterpolator";
    pub const PATH_INTERPOLATOR_CLASS: &str = "android/view/animation/PathInterpolator";
    pub const INTERPOLATOR_CLASS: &str = "android/view/animation/Interpolator";
    pub const ANIMATION_CLASS: &str = "android/view/animation/Animation";
    pub const TRANSLATE_ANIMATION_CLASS: &str = "android/view/animation/TranslateAnimation";
    pub const SCALE_ANIMATION_CLASS: &str = "android/view/animation/ScaleAnimation";
    pub const ROTATE_ANIMATION_CLASS: &str = "android/view/animation/RotateAnimation";
    pub const ALPHA_ANIMATION_CLASS: &str = "android/view/animation/AlphaAnimation";
    pub const ANIMATION_SET_CLASS: &str = "android/view/animation/AnimationSet";
    pub const LAYOUT_ANIMATION_CONTROLLER_CLASS: &str = "android/view/animation/LayoutAnimationController";
    pub const GRID_LAYOUT_ANIMATION_CONTROLLER_CLASS: &str = "android/view/animation/GridLayoutAnimationController";
    pub const VIEW_PROPERTY_ANIMATOR_CLASS: &str = "android/view/ViewPropertyAnimator";
    pub const RENDER_NODE_CLASS: &str = "android/graphics/RenderNode";
    pub const DISPLAY_LIST_CANVAS_CLASS: &str = "android/graphics/DisplayListCanvas";
    pub const HARDWARE_CANVAS_CLASS: &str = "android/graphics/HardwareCanvas";
    pub const BITMAP_SHADER_CLASS: &str = "android/graphics/BitmapShader";
    pub const LINEAR_GRADIENT_CLASS: &str = "android/graphics/LinearGradient";
    pub const RADIAL_GRADIENT_CLASS: &str = "android/graphics/RadialGradient";
    pub const SWEEP_GRADIENT_CLASS: &str = "android/graphics/SweepGradient";
    pub const COMPOSE_SHADER_CLASS: &str = "android/graphics/ComposeShader";
    pub const COLOR_FILTER_CLASS: &str = "android/graphics/ColorFilter";
    pub const LIGHTING_COLOR_FILTER_CLASS: &str = "android/graphics/LightingColorFilter";
    pub const PORTER_DUFF_COLOR_FILTER_CLASS: &str = "android/graphics/PorterDuffColorFilter";
    pub const COLOR_MATRIX_COLOR_FILTER_CLASS: &str = "android/graphics/ColorMatrixColorFilter";
    pub const COLOR_MATRIX_CLASS: &str = "android/graphics/ColorMatrix";
    pub const PATH_EFFECT_CLASS: &str = "android/graphics/PathEffect";
    pub const CORNER_PATH_EFFECT_CLASS: &str = "android/graphics/CornerPathEffect";
    pub const DASH_PATH_EFFECT_CLASS: &str = "android/graphics/DashPathEffect";
    pub const PATH_DASH_PATH_EFFECT_CLASS: &str = "android/graphics/PathDashPathEffect";
    pub const SUM_PATH_EFFECT_CLASS: &str = "android/graphics/SumPathEffect";
    pub const COMPOSE_PATH_EFFECT_CLASS: &str = "android/graphics/ComposePathEffect";
    pub const MASK_FILTER_CLASS: &str = "android/graphics/MaskFilter";
    pub const BLUR_MASK_FILTER_CLASS: &str = "android/graphics/BlurMaskFilter";
    pub const EMBOSS_MASK_FILTER_CLASS: &str = "android/graphics/EmbossMaskFilter";
    pub const SHADER_CLASS: &str = "android/graphics/Shader";
    pub const XFERMODE_CLASS: &str = "android/graphics/Xfermode";
    pub const PORTER_DUFF_XFERMODE_CLASS: &str = "android/graphics/PorterDuffXfermode";
    pub const PORTER_DUFF_MODE: &str = "android/graphics/PorterDuff$Mode";

    pub fn log_debug(tag: &str, msg: &str) {
        let _ = call_static_int_method(
            LOG_CLASS,
            "d",
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&new_string(tag).unwrap()),
                JValue::Object(&new_string(msg).unwrap()),
            ],
        );
    }

    pub fn log_info(tag: &str, msg: &str) {
        let _ = call_static_int_method(
            LOG_CLASS,
            "i",
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&new_string(tag).unwrap()),
                JValue::Object(&new_string(msg).unwrap()),
            ],
        );
    }

    pub fn log_warn(tag: &str, msg: &str) {
        let _ = call_static_int_method(
            LOG_CLASS,
            "w",
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&new_string(tag).unwrap()),
                JValue::Object(&new_string(msg).unwrap()),
            ],
        );
    }

    pub fn log_error(tag: &str, msg: &str) {
        let _ = call_static_int_method(
            LOG_CLASS,
            "e",
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&new_string(tag).unwrap()),
                JValue::Object(&new_string(msg).unwrap()),
            ],
        );
    }

    pub fn show_toast<'a>(context: &JObject<'a>, message: &str, duration: i32) {
        let _ = call_static_object_method(
            TOAST_CLASS,
            "makeText",
            "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;",
            &[
                context.into(),
                JValue::Object(&new_string(message).unwrap()),
                JValue::Int(duration),
            ],
        ).map(|toast| {
            let _ = call_method::<()>(&toast, "show", "()V", &[]);
        });
    }

    pub fn get_system_service<'a>(context: &JObject<'a>, service_name: &str) -> Option<JObject<'a>> {
        call_method(
            context,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&new_string(service_name).unwrap())],
        )
    }

    pub fn start_activity<'a>(context: &JObject<'a>, intent: &JObject<'a>) -> Option<()> {
        call_method(context, "startActivity", "(Landroid/content/Intent;)V", &[intent.into()])
    }

    pub fn create_intent<'a>(action: &str) -> Option<JObject<'a>> {
        new_object(INTENT_CLASS, "(Ljava/lang/String;)V", &[JValue::Object(&new_string(action).unwrap())])
    }

    pub fn create_intent_with_class<'a>(context: &JObject<'a>, class_name: &str) -> Option<JObject<'a>> {
        let class = find_class(class_name)?;
        new_object(INTENT_CLASS, "(Landroid/content/Context;Ljava/lang/Class;)V", &[JValue::Object(context), JValue::Object(&class)])
    }

    pub fn put_extra<'a>(intent: &JObject<'a>, key: &str, value: &str) -> Option<()> {
        call_method(
            intent,
            "putExtra",
            "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Object(&new_string(value).unwrap())],
        ).map(|_: ()| ())
    }

    pub fn put_extra_int<'a>(intent: &JObject<'a>, key: &str, value: i32) -> Option<()> {
        call_method(
            intent,
            "putExtra",
            "(Ljava/lang/String;I)Landroid/content/Intent;",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Int(value)],
        ).map(|_: ()| ())
    }

    pub fn put_extra_long<'a>(intent: &JObject<'a>, key: &str, value: i64) -> Option<()> {
        call_method(
            intent,
            "putExtra",
            "(Ljava/lang/String;J)Landroid/content/Intent;",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Long(value)],
        ).map(|_: ()| ())
    }

    pub fn get_string_extra<'a>(intent: &JObject<'a>, key: &str) -> Option<String> {
        call_method(
            intent,
            "getStringExtra",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JValue::Object(&new_string(key).unwrap())],
        ).and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_int_extra<'a>(intent: &JObject<'a>, key: &str, default: i32) -> Option<i32> {
        call_method(
            intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Int(default)],
        )
    }

    pub fn get_long_extra<'a>(intent: &JObject<'a>, key: &str, default: i64) -> Option<i64> {
        call_method(
            intent,
            "getLongExtra",
            "(Ljava/lang/String;J)J",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Long(default)],
        )
    }

    pub fn get_boolean_extra<'a>(intent: &JObject<'a>, key: &str, default: bool) -> Option<bool> {
        call_method(
            intent,
            "getBooleanExtra",
            "(Ljava/lang/String;Z)Z",
            &[JValue::Object(&new_string(key).unwrap()), JValue::Bool(default as jboolean)],
        )
    }

    pub fn get_package_name<'a>(context: &JObject<'a>) -> Option<String> {
        call_method(
            context,
            "getPackageName",
            "()Ljava/lang/String;",
            &[],
        ).and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_files_dir<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(context, "getFilesDir", "()Ljava/io/File;", &[])
    }

    pub fn get_cache_dir<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(context, "getCacheDir", "()Ljava/io/File;", &[])
    }

    pub fn get_external_files_dir<'a>(context: &JObject<'a>, type_: Option<&str>) -> Option<JObject<'a>> {
        if let Some(t) = type_ {
            call_method(
                context,
                "getExternalFilesDir",
                "(Ljava/lang/String;)Ljava/io/File;",
                &[JValue::Object(&new_string(t).unwrap())],
            )
        } else {
            call_method(context, "getExternalFilesDir", "(Ljava/lang/String;)Ljava/io/File;", &[JValue::Object(&jni::objects::JObject::null())])
        }
    }

    pub fn get_shared_preferences<'a>(context: &JObject<'a>, name: &str, mode: i32) -> Option<JObject<'a>> {
        call_method(
            context,
            "getSharedPreferences",
            "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
            &[JValue::Object(&new_string(name).unwrap()), JValue::Int(mode)],
        )
    }

    pub fn get_content_resolver<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(context, "getContentResolver", "()Landroid/content/ContentResolver;", &[])
    }

    pub fn get_resources<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(context, "getResources", "()Landroid/content/res/Resources;", &[])
    }

    pub fn get_package_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(context, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[])
    }

    pub fn vibrate<'a>(vibrator: &JObject<'a>, milliseconds: i64) -> Option<()> {
        call_method(vibrator, "vibrate", "(J)V", &[JValue::Long(milliseconds)])
    }

    pub fn vibrate_pattern<'a>(vibrator: &JObject<'a>, pattern: &[i64], repeat: i32) -> Option<()> {
        let mut env = get_jvm()?.get_env().ok()?;
        let long_array = env.new_long_array(pattern.len() as i32).ok()?;
        env.set_long_array_region(&long_array, 0, pattern).ok()?;
        call_method(vibrator, "vibrate", "([JI)V", &[JValue::Object(&long_array), JValue::Int(repeat)])
    }

    pub fn has_vibrator<'a>(vibrator: &JObject<'a>) -> Option<bool> {
        call_method(vibrator, "hasVibrator", "()Z", &[])
    }

    pub fn cancel_vibration<'a>(vibrator: &JObject<'a>) -> Option<()> {
        call_method(vibrator, "cancel", "()V", &[])
    }

    pub fn get_screen_width<'a>(context: &JObject<'a>) -> Option<i32> {
        let resources = get_resources(context)?;
        let display_metrics = call_method(&resources, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;", &[])?;
        get_field(&display_metrics, "widthPixels", "I")
    }

    pub fn get_screen_height<'a>(context: &JObject<'a>) -> Option<i32> {
        let resources = get_resources(context)?;
        let display_metrics = call_method(&resources, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;", &[])?;
        get_field(&display_metrics, "heightPixels", "I")
    }

    pub fn get_density<'a>(context: &JObject<'a>) -> Option<f32> {
        let resources = get_resources(context)?;
        let display_metrics = call_method(&resources, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;", &[])?;
        get_field(&display_metrics, "density", "F")
    }

    pub fn get_density_dpi<'a>(context: &JObject<'a>) -> Option<i32> {
        let resources = get_resources(context)?;
        let display_metrics = call_method(&resources, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;", &[])?;
        get_field(&display_metrics, "densityDpi", "I")
    }

    pub fn get_orientation<'a>(context: &JObject<'a>) -> Option<i32> {
        let resources = get_resources(context)?;
        call_method(&resources, "getConfiguration", "()Landroid/content/res/Configuration;", &[])
            .and_then(|config| get_field(&config, "orientation", "I"))
    }

    pub fn is_tablet<'a>(context: &JObject<'a>) -> Option<bool> {
        let resources = get_resources(context)?;
        let config = call_method(&resources, "getConfiguration", "()Landroid/content/res/Configuration;", &[])?;
        let screen_layout: i32 = get_field(&config, "screenLayout", "I")?;
        let size_mask = screen_layout & 0x0F;
        Some(size_mask >= 3)
    }

    pub fn get_version_sdk_int() -> Option<i32> {
        call_static_int_method(VERSION_CLASS, "SDK_INT", "I", &[])
    }

    pub fn get_device_model() -> Option<String> {
        call_static_string_method(BUILD_CLASS, "MODEL", "Ljava/lang/String;", &[])
    }

    pub fn get_device_manufacturer() -> Option<String> {
        call_static_string_method(BUILD_CLASS, "MANUFACTURER", "Ljava/lang/String;", &[])
    }

    pub fn get_device_brand() -> Option<String> {
        call_static_string_method(BUILD_CLASS, "BRAND", "Ljava/lang/String;", &[])
    }

    pub fn get_device_product() -> Option<String> {
        call_static_string_method(BUILD_CLASS, "PRODUCT", "Ljava/lang/String;", &[])
    }

pub fn get_device_id<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getDeviceId", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_sim_serial_number<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getSimSerialNumber", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_line1_number<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getLine1Number", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_network_operator<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getNetworkOperator", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_network_operator_name<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getNetworkOperatorName", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_phone_type<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getPhoneType", "()I", &[])
    }

    pub fn is_network_roaming<'a>(context: &JObject<'a>) -> Option<bool> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "isNetworkRoaming", "()Z", &[])
    }

    pub fn get_cell_location<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getCellLocation", "()Landroid/telephony/CellLocation;", &[])
    }

    pub fn get_all_cell_info<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getAllCellInfo", "()Ljava/util/List;", &[])
    }

    pub fn get_signal_strength<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getSignalStrength", "()Landroid/telephony/SignalStrength;", &[])
    }

    pub fn get_voicemail_number<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVoiceMailNumber", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_voicemail_alpha_tag<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVoiceMailAlphaTag", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_subscriber_id<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getSubscriberId", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_imei<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getImei", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_meid<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getMeid", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn is_radio_on<'a>(context: &JObject<'a>) -> Option<bool> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "isRadioOn", "()Z", &[])
    }

    pub fn get_call_state<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getCallState", "()I", &[])
    }

    pub fn get_data_state<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getDataState", "()I", &[])
    }

    pub fn get_data_activity<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getDataActivity", "()I", &[])
    }

    pub fn get_voice_message_count<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVoiceMessageCount", "()I", &[])
    }

    pub fn get_voicemail_notification_enabled<'a>(context: &JObject<'a>) -> Option<bool> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "isVoicemailNotificationEnabled", "()Z", &[])
    }

    pub fn get_visual_voicemail_package_name<'a>(context: &JObject<'a>) -> Option<String> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVisualVoicemailPackageName", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_visual_voicemail_sms_filter_settings<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVisualVoicemailSmsFilterSettings", "()Landroid/telephony/VisualVoicemailSmsFilterSettings;", &[])
    }

    pub fn get_carrier_configs<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getCarrierConfigValues", "()Landroid/os/PersistableBundle;", &[])
    }

    pub fn get_visual_voicemail_transcription_status<'a>(context: &JObject<'a>) -> Option<i32> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "getVisualVoicemailTranscriptionStatus", "()I", &[])
    }

    pub fn is_voice_capable<'a>(context: &JObject<'a>) -> Option<bool> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "isVoiceCapable", "()Z", &[])
    }

    pub fn is_sms_capable<'a>(context: &JObject<'a>) -> Option<bool> {
        let telephony = get_system_service(context, "phone")?;
        call_method(&telephony, "isSmsCapable", "()Z", &[])
    }

    pub fn get_sms_messages<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        let resolver = get_content_resolver(context)?;
        let uri = new_string("content://sms/")?;
        let uri_obj = new_object("android/net/Uri", "(Ljava/lang/String;)V", &[JValue::Object(&uri)])?;
        call_method(&resolver, "query", "(Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)Landroid/database/Cursor;", &[
            JValue::Object(&uri_obj),
            JValue::Object(&jni::objects::JObject::null()),
            JValue::Object(&jni::objects::JObject::null()),
            JValue::Object(&jni::objects::JObject::null()),
            JValue::Object(&jni::objects::JObject::null()),
        ])
    }

    pub fn send_sms<'a>(context: &JObject<'a>, destination: &str, text: &str) -> Option<()> {
        let sms_manager = call_static_object_method(
            "android/telephony/SmsManager",
            "getDefault",
            "()Landroid/telephony/SmsManager;",
            &[],
        )?;
        call_method(
            &sms_manager,
            "sendTextMessage",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Landroid/app/PendingIntent;Landroid/app/PendingIntent;)V",
            &[
                JValue::Object(&new_string(destination).unwrap()),
                JValue::Object(&jni::objects::JObject::null()),
                JValue::Object(&new_string(text).unwrap()),
                JValue::Object(&jni::objects::JObject::null()),
                JValue::Object(&jni::objects::JObject::null()),
            ],
        ).map(|_: ()| ())
    }

    pub fn send_multipart_sms<'a>(context: &JObject<'a>, destination: &str, parts: &[String]) -> Option<()> {
        let sms_manager = call_static_object_method(
            "android/telephony/SmsManager",
            "getDefault",
            "()Landroid/telephony/SmsManager;",
            &[],
        )?;
        let mut env = get_jvm()?.get_env().ok()?;
        let array = env.new_object_array(parts.len() as i32, STRING_CLASS, JObject::null()).ok()?;
        for (i, part) in parts.iter().enumerate() {
            let jstr = new_string(part)?;
            env.set_object_array_element(&array, i as i32, &jstr).ok()?;
        }
        call_method(
            &sms_manager,
            "sendMultipartTextMessage",
            "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;[Landroid/app/PendingIntent;[Landroid/app/PendingIntent;)V",
            &[
                JValue::Object(&new_string(destination).unwrap()),
                JValue::Object(&jni::objects::JObject::null()),
                JValue::Object(&array),
                JValue::Object(&jni::objects::JObject::null()),
                JValue::Object(&jni::objects::JObject::null()),
            ],
        ).map(|_: ()| ())
    }

    pub fn get_bluetooth_adapter() -> Option<JObject<'static>> {
        call_static_object_method(
            BLUETOOTH_ADAPTER_CLASS,
            "getDefaultAdapter",
            "()Landroid/bluetooth/BluetoothAdapter;",
            &[],
        )
    }

    pub fn is_bluetooth_enabled<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "isEnabled", "()Z", &[])
    }

    pub fn enable_bluetooth<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "enable", "()Z", &[])
    }

    pub fn disable_bluetooth<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "disable", "()Z", &[])
    }

    pub fn get_bluetooth_name<'a>(adapter: &JObject<'a>) -> Option<String> {
        call_method(adapter, "getName", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn set_bluetooth_name<'a>(adapter: &JObject<'a>, name: &str) -> Option<bool> {
        call_method(
            adapter,
            "setName",
            "(Ljava/lang/String;)Z",
            &[JValue::Object(&new_string(name).unwrap())],
        )
    }

    pub fn get_bluetooth_address<'a>(adapter: &JObject<'a>) -> Option<String> {
        call_method(adapter, "getAddress", "()Ljava/lang/String;", &[])
            .and_then(|obj: jni::objects::JObject<'_>| get_string(&jni::objects::JString::from(obj)))
    }

    pub fn get_bonded_devices<'a>(adapter: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(adapter, "getBondedDevices", "()Ljava/util/Set;", &[])
    }

    pub fn start_discovery<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "startDiscovery", "()Z", &[])
    }

    pub fn cancel_discovery<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "cancelDiscovery", "()Z", &[])
    }

    pub fn is_discovering<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "isDiscovering", "()Z", &[])
    }

    pub fn get_scan_mode<'a>(adapter: &JObject<'a>) -> Option<i32> {
        call_method(adapter, "getScanMode", "()I", &[])
    }

    pub fn set_scan_mode<'a>(adapter: &JObject<'a>, mode: i32, duration: i32) -> Option<bool> {
        call_method(
            adapter,
            "setScanMode",
            "(II)Z",
            &[JValue::Int(mode), JValue::Int(duration)],
        )
    }

    pub fn get_bluetooth_le_advertiser<'a>(adapter: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(adapter, "getBluetoothLeAdvertiser", "()Landroid/bluetooth/le/BluetoothLeAdvertiser;", &[])
    }

    pub fn get_bluetooth_le_scanner<'a>(adapter: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(adapter, "getBluetoothLeScanner", "()Landroid/bluetooth/le/BluetoothLeScanner;", &[])
    }

    pub fn get_nfc_adapter<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        call_static_object_method(
            NFC_ADAPTER_CLASS,
            "getDefaultAdapter",
            "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;",
            &[context.into()],
        )
    }

    pub fn is_nfc_enabled<'a>(adapter: &JObject<'a>) -> Option<bool> {
        call_method(adapter, "isEnabled", "()Z", &[])
    }

    pub fn enable_nfc_foreground_dispatch<'a>(adapter: &JObject<'a>, activity: &JObject<'a>, filters: &JObject<'a>, tech_lists: &JObject<'a>) -> Option<()> {
        call_method(
            adapter,
            "enableForegroundDispatch",
            "(Landroid/app/Activity;[Landroid/content/IntentFilter;[[Ljava/lang/String;)V",
            &[activity.into(), filters.into(), tech_lists.into()],
        ).map(|_: ()| ())
    }

    pub fn disable_nfc_foreground_dispatch<'a>(adapter: &JObject<'a>, activity: &JObject<'a>) -> Option<()> {
        call_method(
            adapter,
            "disableForegroundDispatch",
            "(Landroid/app/Activity;)V",
            &[activity.into()],
        ).map(|_: ()| ())
    }

    pub fn get_nfc_tag<'a>(intent: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(
            intent,
            "getParcelableExtra",
            "(Ljava/lang/String;)Landroid/os/Parcelable;",
            &[JValue::Object(&new_string("android.nfc.extra.TAG").unwrap())],
        )
    }

    pub fn get_nfc_ndef_messages<'a>(intent: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(
            intent,
            "getParcelableArrayExtra",
            "(Ljava/lang/String;)[Landroid/os/Parcelable;",
            &[JValue::Object(&new_string("android.nfc.extra.NDEF_MESSAGES").unwrap())],
        )
    }

    pub fn get_location_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "location")
    }

    pub fn get_last_known_location<'a>(manager: &JObject<'a>, provider: &str) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getLastKnownLocation",
            "(Ljava/lang/String;)Landroid/location/Location;",
            &[JValue::Object(&new_string(provider).unwrap())],
        )
    }

    pub fn request_location_updates<'a>(
        manager: &JObject<'a>,
        provider: &str,
        min_time: i64,
        min_distance: f32,
        listener: &JObject<'a>,
    ) -> Option<()> {
        call_method(
            manager,
            "requestLocationUpdates",
            "(Ljava/lang/String;JFLandroid/location/LocationListener;)V",
            &[
                JValue::Object(&new_string(provider).unwrap()),
                JValue::Long(min_time),
                JValue::Float(min_distance),
                listener.into(),
            ],
        ).map(|_: ()| ())
    }

    pub fn remove_location_updates<'a>(manager: &JObject<'a>, listener: &JObject<'a>) -> Option<()> {
        call_method(
            manager,
            "removeUpdates",
            "(Landroid/location/LocationListener;)V",
            &[listener.into()],
        ).map(|_: ()| ())
    }

    pub fn get_providers<'a>(manager: &JObject<'a>, enabled_only: bool) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getProviders",
            "(Z)Ljava/util/List;",
            &[JValue::Bool(enabled_only as jboolean)],
        )
    }

    pub fn is_provider_enabled<'a>(manager: &JObject<'a>, provider: &str) -> Option<bool> {
        call_method(
            manager,
            "isProviderEnabled",
            "(Ljava/lang/String;)Z",
            &[JValue::Object(&new_string(provider).unwrap())],
        )
    }

    pub fn get_gps_status<'a>(manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(manager, "getGpsStatus", "(Landroid/location/GpsStatus;)Landroid/location/GpsStatus;", &[JValue::Object(&jni::objects::JObject::null())])
    }

    pub fn get_camera_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "camera")
    }

    pub fn get_camera_id_list<'a>(manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(manager, "getCameraIdList", "()[Ljava/lang/String;", &[])
    }

    pub fn get_camera_characteristics<'a>(manager: &JObject<'a>, camera_id: &str) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getCameraCharacteristics",
            "(Ljava/lang/String;)Landroid/hardware/camera2/CameraCharacteristics;",
            &[JValue::Object(&new_string(camera_id).unwrap())],
        )
    }

    pub fn open_camera<'a>(manager: &JObject<'a>, camera_id: &str, callback: &JObject<'a>, handler: &JObject<'a>) -> Option<()> {
        call_method(
            manager,
            "openCamera",
            "(Ljava/lang/String;Landroid/hardware/camera2/CameraDevice$StateCallback;Landroid/os/Handler;)V",
            &[JValue::Object(&new_string(camera_id).unwrap()), callback.into(), handler.into()],
        ).map(|_: ()| ())
    }

    pub fn get_sensor_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "sensor")
    }

    pub fn get_sensor_list<'a>(manager: &JObject<'a>, type_: i32) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getSensorList",
            "(I)Ljava/util/List;",
            &[JValue::Int(type_)],
        )
    }

    pub fn get_default_sensor<'a>(manager: &JObject<'a>, type_: i32) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getDefaultSensor",
            "(I)Landroid/hardware/Sensor;",
            &[JValue::Int(type_)],
        )
    }

    pub fn register_listener<'a>(
        manager: &JObject<'a>,
        listener: &JObject<'a>,
        sensor: &JObject<'a>,
        rate: i32,
    ) -> Option<bool> {
        call_method(
            manager,
            "registerListener",
            "(Landroid/hardware/SensorEventListener;Landroid/hardware/Sensor;I)Z",
            &[listener.into(), sensor.into(), JValue::Int(rate)],
        )
    }

    pub fn unregister_listener<'a>(manager: &JObject<'a>, listener: &JObject<'a>) -> Option<()> {
        call_method(
            manager,
            "unregisterListener",
            "(Landroid/hardware/SensorEventListener;)V",
            &[listener.into()],
        ).map(|_: ()| ())
    }

    pub fn get_audio_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "audio")
    }

    pub fn get_stream_volume<'a>(audio_manager: &JObject<'a>, stream_type: i32) -> Option<i32> {
        call_method(
            audio_manager,
            "getStreamVolume",
            "(I)I",
            &[JValue::Int(stream_type)],
        )
    }

    pub fn set_stream_volume<'a>(audio_manager: &JObject<'a>, stream_type: i32, index: i32, flags: i32) -> Option<()> {
        call_method(
            audio_manager,
            "setStreamVolume",
            "(III)V",
            &[JValue::Int(stream_type), JValue::Int(index), JValue::Int(flags)],
        ).map(|_: ()| ())
    }

    pub fn get_stream_max_volume<'a>(audio_manager: &JObject<'a>, stream_type: i32) -> Option<i32> {
        call_method(
            audio_manager,
            "getStreamMaxVolume",
            "(I)I",
            &[JValue::Int(stream_type)],
        )
    }

    pub fn is_stream_mute<'a>(audio_manager: &JObject<'a>, stream_type: i32) -> Option<bool> {
        call_method(
            audio_manager,
            "isStreamMute",
            "(I)Z",
            &[JValue::Int(stream_type)],
        )
    }

    pub fn set_stream_mute<'a>(audio_manager: &JObject<'a>, stream_type: i32, state: bool) -> Option<()> {
        call_method(
            audio_manager,
            "setStreamMute",
            "(IZ)V",
            &[JValue::Int(stream_type), JValue::Bool(state as jboolean)],
        ).map(|_: ()| ())
    }

    pub fn get_mode<'a>(audio_manager: &JObject<'a>) -> Option<i32> {
        call_method(audio_manager, "getMode", "()I", &[])
    }

    pub fn set_mode<'a>(audio_manager: &JObject<'a>, mode: i32) -> Option<()> {
        call_method(
            audio_manager,
            "setMode",
            "(I)V",
            &[JValue::Int(mode)],
        ).map(|_: ()| ())
    }

    pub fn get_ringer_mode<'a>(audio_manager: &JObject<'a>) -> Option<i32> {
        call_method(audio_manager, "getRingerMode", "()I", &[])
    }

    pub fn set_ringer_mode<'a>(audio_manager: &JObject<'a>, mode: i32) -> Option<()> {
        call_method(
            audio_manager,
            "setRingerMode",
            "(I)V",
            &[JValue::Int(mode)],
        ).map(|_: ()| ())
    }

    pub fn is_music_active<'a>(audio_manager: &JObject<'a>) -> Option<bool> {
        call_method(audio_manager, "isMusicActive", "()Z", &[])
    }

    pub fn is_speakerphone_on<'a>(audio_manager: &JObject<'a>) -> Option<bool> {
        call_method(audio_manager, "isSpeakerphoneOn", "()Z", &[])
    }

    pub fn set_speakerphone_on<'a>(audio_manager: &JObject<'a>, on: bool) -> Option<()> {
        call_method(
            audio_manager,
            "setSpeakerphoneOn",
            "(Z)V",
            &[JValue::Bool(on as jboolean)],
        ).map(|_: ()| ())
    }

    pub fn get_microphones<'a>(audio_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(audio_manager, "getMicrophones", "()[Landroid/media/AudioDeviceInfo;", &[])
    }

    pub fn get_devices<'a>(audio_manager: &JObject<'a>, flags: i32) -> Option<JObject<'a>> {
        call_method(
            audio_manager,
            "getDevices",
            "(I)[Landroid/media/AudioDeviceInfo;",
            &[JValue::Int(flags)],
        )
    }

    pub fn get_connectivity_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "connectivity")
    }

    pub fn get_active_network_info<'a>(manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(manager, "getActiveNetworkInfo", "()Landroid/net/NetworkInfo;", &[])
    }

    pub fn get_all_network_info<'a>(manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(manager, "getAllNetworkInfo", "()[Landroid/net/NetworkInfo;", &[])
    }

    pub fn get_network_info<'a>(manager: &JObject<'a>, network: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getNetworkInfo",
            "(Landroid/net/Network;)Landroid/net/NetworkInfo;",
            &[network.into()],
        )
    }

    pub fn get_all_networks<'a>(manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(manager, "getAllNetworks", "()[Landroid/net/Network;", &[])
    }

    pub fn request_network<'a>(manager: &JObject<'a>, request: &JObject<'a>, callback: &JObject<'a>) -> Option<()> {
        call_method(
            manager,
            "requestNetwork",
            "(Landroid/net/NetworkRequest;Landroid/net/ConnectivityManager$NetworkCallback;)V",
            &[request.into(), callback.into()],
        ).map(|_: ()| ())
    }

    pub fn unregister_network_callback<'a>(manager: &JObject<'a>, callback: &JObject<'a>) -> Option<()> {
        call_method(
            manager,
            "unregisterNetworkCallback",
            "(Landroid/net/ConnectivityManager$NetworkCallback;)V",
            &[callback.into()],
        ).map(|_: ()| ())
    }

    pub fn is_active_network_metered<'a>(manager: &JObject<'a>) -> Option<bool> {
        call_method(manager, "isActiveNetworkMetered", "()Z", &[])
    }

    pub fn get_restrict_background_status<'a>(manager: &JObject<'a>) -> Option<i32> {
        call_method(manager, "getRestrictBackgroundStatus", "()I", &[])
    }

    pub fn is_network_supported<'a>(manager: &JObject<'a>, network: &JObject<'a>) -> Option<bool> {
        call_method(
            manager,
            "isNetworkSupported",
            "(Landroid/net/Network;)Z",
            &[network.into()],
        )
    }

    pub fn get_link_properties<'a>(manager: &JObject<'a>, network: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getLinkProperties",
            "(Landroid/net/Network;)Landroid/net/LinkProperties;",
            &[network.into()],
        )
    }

    pub fn get_network_capabilities<'a>(manager: &JObject<'a>, network: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(
            manager,
            "getNetworkCapabilities",
            "(Landroid/net/Network;)Landroid/net/NetworkCapabilities;",
            &[network.into()],
        )
    }

    pub fn get_wifi_manager<'a>(context: &JObject<'a>) -> Option<JObject<'a>> {
        get_system_service(context, "wifi")
    }

    pub fn is_wifi_enabled<'a>(wifi_manager: &JObject<'a>) -> Option<bool> {
        call_method(wifi_manager, "isWifiEnabled", "()Z", &[])
    }

    pub fn set_wifi_enabled<'a>(wifi_manager: &JObject<'a>, enabled: bool) -> Option<bool> {
        call_method(
            wifi_manager,
            "setWifiEnabled",
            "(Z)Z",
            &[JValue::Bool(enabled as jboolean)],
        )
    }

    pub fn get_wifi_state<'a>(wifi_manager: &JObject<'a>) -> Option<i32> {
        call_method(wifi_manager, "getWifiState", "()I", &[])
    }

    pub fn get_configured_networks<'a>(wifi_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(wifi_manager, "getConfiguredNetworks", "()Ljava/util/List;", &[])
    }

    pub fn add_network<'a>(wifi_manager: &JObject<'a>, config: &JObject<'a>) -> Option<i32> {
        call_method(
            wifi_manager,
            "addNetwork",
            "(Landroid/net/wifi/WifiConfiguration;)I",
            &[config.into()],
        )
    }

    pub fn remove_network<'a>(wifi_manager: &JObject<'a>, net_id: i32) -> Option<bool> {
        call_method(
            wifi_manager,
            "removeNetwork",
            "(I)Z",
            &[JValue::Int(net_id)],
        )
    }

    pub fn enable_network<'a>(wifi_manager: &JObject<'a>, net_id: i32, disable_others: bool) -> Option<bool> {
        call_method(
            wifi_manager,
            "enableNetwork",
            "(IZ)Z",
            &[JValue::Int(net_id), JValue::Bool(disable_others as jboolean)],
        )
    }

    pub fn disable_network<'a>(wifi_manager: &JObject<'a>, net_id: i32) -> Option<bool> {
        call_method(
            wifi_manager,
            "disableNetwork",
            "(I)Z",
            &[JValue::Int(net_id)],
        )
    }

    pub fn reconnect<'a>(wifi_manager: &JObject<'a>) -> Option<bool> {
        call_method(wifi_manager, "reconnect", "()Z", &[])
    }

    pub fn reassociate<'a>(wifi_manager: &JObject<'a>) -> Option<bool> {
        call_method(wifi_manager, "reassociate", "()Z", &[])
    }

    pub fn start_scan<'a>(wifi_manager: &JObject<'a>) -> Option<bool> {
        call_method(wifi_manager, "startScan", "()Z", &[])
    }

    pub fn get_scan_results<'a>(wifi_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(wifi_manager, "getScanResults", "()Ljava/util/List;", &[])
    }

    pub fn get_connection_info<'a>(wifi_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(wifi_manager, "getConnectionInfo", "()Landroid/net/wifi/WifiInfo;", &[])
    }

    pub fn create_wifi_lock<'a>(wifi_manager: &JObject<'a>, lock_type: i32, tag: &str) -> Option<JObject<'a>> {
        call_method(
            wifi_manager,
            "createWifiLock",
            "(ILjava/lang/String;)Landroid/net/wifi/WifiManager$WifiLock;",
            &[JValue::Int(lock_type), JValue::Object(&new_string(tag).unwrap())],
        )
    }

    pub fn create_multicast_lock<'a>(wifi_manager: &JObject<'a>, tag: &str) -> Option<JObject<'a>> {
        call_method(
            wifi_manager,
            "createMulticastLock",
            "(Ljava/lang/String;)Landroid/net/wifi/WifiManager$MulticastLock;",
            &[JValue::Object(&new_string(tag).unwrap())],
        )
    }

    pub fn get_dhcp_info<'a>(wifi_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(wifi_manager, "getDhcpInfo", "()Landroid/net/DhcpInfo;", &[])
    }

    pub fn set_wifi_ap_enabled<'a>(wifi_manager: &JObject<'a>, config: &JObject<'a>, enabled: bool) -> Option<bool> {
        call_method(
            wifi_manager,
            "setWifiApEnabled",
            "(Landroid/net/wifi/WifiConfiguration;Z)Z",
            &[config.into(), JValue::Bool(enabled as jboolean)],
        )
    }

    pub fn get_wifi_ap_state<'a>(wifi_manager: &JObject<'a>) -> Option<i32> {
        call_method(wifi_manager, "getWifiApState", "()I", &[])
    }

    pub fn get_wifi_ap_configuration<'a>(wifi_manager: &JObject<'a>) -> Option<JObject<'a>> {
        call_method(wifi_manager, "getWifiApConfiguration", "()Landroid/net/wifi/WifiConfiguration;", &[])
    }

    pub fn set_wifi_ap_configuration<'a>(wifi_manager: &JObject<'a>, config: &JObject<'a>) -> Option<bool> {
        call_method(
            wifi_manager,
            "setWifiApConfiguration",
            "(Landroid/net/wifi/WifiConfiguration;)Z",
            &[config.into()],
        )
    }
}
